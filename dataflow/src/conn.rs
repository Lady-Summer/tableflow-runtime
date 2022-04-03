use std::time;
use tokio::sync::mpsc;
use crate::{err, event, types};
use crate::event::Event;

const DEFAULT_EVENT_TIME_DURATION: time::Duration = time::Duration::from_millis(1);

pub enum ConnectorType {
    Tableflow {
        limit: u32,
        uri: String,
    },
    Redis {
        conn: redis::Connection,
    },
}

pub struct Connector {
    pub binders: Vec<types::Binder>,
    pub connector_type: ConnectorType,
    event_rx: mpsc::UnboundedReceiver<Vec<event::BinderEvent>>,
    disconnect_rx: mpsc::Receiver<event::Disconnect>,
    event_time_duration: Option<time::Duration>,
}

impl Connector {
    pub fn new(desc: &types::SourceDesc,
               event_rx: mpsc::UnboundedReceiver<Vec<event::BinderEvent>>,
               disconnect_rx: mpsc::Receiver<event::Disconnect>) -> Connector {
        match desc {
            types::SourceDesc::Tableflow { host, port, limit, event_time } =>
                Connector {
                    binders: vec![],
                    connector_type: ConnectorType::Tableflow {
                        limit: limit.clone(),
                        uri: format!("{}:{}", host, port),
                    },
                    event_rx,
                    disconnect_rx,
                    event_time_duration: event_time.map(|secs| time::Duration::from_secs(secs)),
                },
            types::SourceDesc::Redis {
                host, port,
                username, password,
                db
            } => Connector {
                binders: vec![],
                connector_type: ConnectorType::Redis {
                    conn: redis::Client::open(
                        redis::ConnectionInfo {
                            addr: redis::ConnectionAddr::Tcp(host.clone(), port.clone()),
                            redis: redis::RedisConnectionInfo {
                                db: db.clone() as i64,
                                username: username.clone(),
                                password: password.clone(),
                            },
                        })
                        .expect("invalid redis connector source describe")
                        .get_connection()
                        .expect("invalid connect redis"),
                },
                event_rx,
                disconnect_rx,
                event_time_duration: None,
            }
        }
    }

    fn handle_event(self: &mut Self, e: &event::BinderEvent) {
        match &e.binder_type {
            event::BinderEventType::Create {
                table_id,
                header_id,
                id,
                addr
            } => match &mut self.connector_type {
                ConnectorType::Redis { conn } => {
                    let b = types::Binder {
                        job_id: e.job_id.clone(),
                        binder_type: types::BinderType::Redis,
                        table_id: table_id.clone(),
                        header_id: header_id.to_string(),
                        id: id.clone(),
                        addr: addr.clone(),
                    };
                    let _ = conn.as_pubsub()
                        .subscribe(b.get_topic());

                    self.binders.push(b)
                }
                ConnectorType::Tableflow { .. } => self.binders.push(
                    types::Binder {
                        job_id: e.job_id.clone(),
                        binder_type: types::BinderType::Tableflow { page: 0 },
                        table_id: table_id.clone(),
                        header_id: header_id.to_string(),
                        id: id.clone(),
                        addr: addr.clone(),
                    }
                )
            },
            event::BinderEventType::Stop => {
                match &mut self.connector_type {
                    ConnectorType::Redis { conn } => {
                        let topics = core::lists::filter_map(
                            &self.binders,
                            |binder| binder.job_id.eq(&e.job_id),
                            |binder| binder.get_topic(),
                        );

                        let _ = conn.as_pubsub()
                            .unsubscribe(topics);
                    }
                    _ => {}
                }

                core::lists::remove_if(&mut self.binders, |binder| binder.job_id.eq(&e.job_id));
            }
        }
    }

    pub async fn fetch(&mut self) -> Option<event::ConnectorEvent> {
        match &mut self.connector_type {
            ConnectorType::Redis { conn } => {
                match conn.as_pubsub().get_message() {
                    Ok(msg) => None,
                    Err(err) => {
                        log::error!("fail to get message: {:?}", err);
                        None
                    }
                }
            }
            _ => None
        }
    }

    pub async fn start(mut self) {
        let ref mut connector = self;
        let mut ticker = tokio::time::interval(
            connector
                .event_time_duration
                .unwrap_or(DEFAULT_EVENT_TIME_DURATION)
        );

        let (tx, mut rx) = mpsc::unbounded_channel();

        loop {
            match connector.fetch().await {
                None => {}
                Some(event) => {
                    let _ = tx.send(event)
                        .map_err(|err| {
                            log::error!("send event failed: {}", &err);
                            err
                        });
                }
            }

            tokio::select! {
                Some(_) = connector.disconnect_rx.recv() => break,
                Some(events) = connector.event_rx.recv() => core::lists::for_each(&events, |e| connector.handle_event(e)),
                _ = ticker.tick(), if connector.is_tableflow() => {},
                Some(event) = rx.recv() => send_to_worker(event)
            }
        }

        connector.disconnect_rx.close();
        connector.event_rx.close();
    }

    fn is_tableflow(&self) -> bool {
        match &self.connector_type {
            ConnectorType::Tableflow { .. } => true,
            _ => false
        }
    }
}

fn send_to_worker(event: event::ConnectorEvent) {
    let ref clients = core::lists::map(
        &event.binders,
        |binder| dataflow_api::worker::new_dataflow_worker_client(
            dataflow_api::worker::DataflowWorkerConfig {
                host: None,
                port: None,
                uri: Some(binder.addr.clone()),
            }
        ),
    );

    let event_type = event::FormulaOpEventType::from(&event.event_type);
    let event_time = time::SystemTime::now();

    core::lists::index_for_each(clients, |idx, cli| {
        let ref mut request = dataflow_api::dataflow_worker::ActionSubmitRequest::new();
        let b = &event.binders[idx];
        let ref graph_event = event::GraphEvent::NodeEventSubmit(
            event::FormulaOpEvent {
                job_id: event.get_key(),
                from: 0,
                to: b.id.clone(),
                event_type: event_type.clone(),
                data: event.entries.to_vec(),
                event_time: event_time.clone(),
            }
        );

        let _ = serde_json::to_string(graph_event)
            .map_err(|err| err::CommonException::from(err))
            .and_then(|value| {
                request.set_value(value.as_bytes().to_vec());
                cli.submit_action(request)
                    .map(|resp| {
                        log::debug!("send action event success")
                    })
                    .map_err(|err| err::CommonException::from(err))
            })
            .map_err(|err| {
                log::error!("serialize failed: {:?}", err);
                err
            });
    })
}