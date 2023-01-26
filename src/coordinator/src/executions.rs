use std::{
    collections::BTreeMap,
    sync::atomic::{self, AtomicU64},
};

use common::{
    net::{
        cluster::{Node, NodeStatus},
        gateway::{worker::SafeTaskManagerRpcGateway, ReceiveAckRpcGateway},
        to_host_addr, AckResponder, AckResponderBuilder, PersistableHostAddr,
    },
    types::ExecutorId,
    utils::{self, times::from_utc_chrono_to_prost_timestamp},
    ExecutionID,
};
use proto::{
    common::{
        ack::{AckType, RequestId},
        Ack, Dataflow, DataflowStatus, Heartbeat, NodeType, OperatorInfo,
    },
    worker::CreateSubDataflowRequest,
};
use tokio::{sync::mpsc, task::JoinHandle};

/// This module contains all logical execution contexts of a dataflow, an operator or an edge which are running on the remote TaskManager node.
/// These contexts contains data which can reflect the inner state of the dataflows, operators and edges such as running or not, checkpoint status.
///
/// # Observability
///
/// Execution must be observable in a cloud environment to help developers to know what happens in a running dataflow or operator.
/// If anything wrong happens, they can be informed as soon as possible and take the actions.
///
/// Mainstream cloud-observability systems like Prometheus are using three kinds of data:
/// - Metrics
/// - Tracing
/// - Logs
///
/// In 1.0 release version, Lightflus will periodically request/report these data and store them in Coordinator/TaskManger.
/// Users can configure to dump them into an outside system like ES automatically.
///
/// # Fault Tolerance
///
/// # High Availability
///
///

/// A [`VertexExecution`] represents an execution context for a [`LocalExecutor`].
/// - watch each LocalExecutor's state details
/// - restart LocalExecutor while it stops unexpectedly
/// - watch each LocalExecutor's checkpoint snapshot status
/// - collect each LocalExecutor's metrics
pub(crate) struct VertexExecution {
    executor_id: ExecutorId,
    operator: OperatorInfo,
}

impl VertexExecution {
    pub(crate) fn new(executor_id: &ExecutorId, operator: &OperatorInfo) -> Self {
        todo!()
    }
}

/// A [`SubdataflowDeploymentPlan`] represents a description for a subdataflow [`Dataflow`] deployment. It may contains following properties:
/// - the structure of subdataflow
/// - the execution id of the subdataflow
/// - the resource configurations that this dataflow can be allocated
/// - ack responder
/// - initialized ack request queue
pub(crate) struct SubdataflowDeploymentPlan {
    /// the description of subdataflow
    subdataflow: Dataflow,
    /// the target address of TaskManager
    addr: PersistableHostAddr,
    /// the id of the subdataflow's execution
    execution_id: ExecutionID,
    /// the gateway of TaskManager
    gateway: Option<SafeTaskManagerRpcGateway>,
    /// ack responder
    ack: AckResponder<SafeTaskManagerRpcGateway>,
    /// the initialized enqueue-entrypoint of a ack request queue
    sender: mpsc::Sender<Ack>,
}
impl SubdataflowDeploymentPlan {
    pub(crate) fn new(
        subdataflow: (&PersistableHostAddr, &Dataflow),
        execution_id: ExecutionID,
        node: Option<&Node>,
        connect_timeout: u64,
        rpc_timout: u64,
        ack_builder: &AckResponderBuilder,
    ) -> Self {
        let (ack, sender) =
            ack_builder.build(|addr| SafeTaskManagerRpcGateway::new(&to_host_addr(addr)));
        Self {
            subdataflow: subdataflow.1.clone(),
            addr: subdataflow.0.clone(),
            execution_id,
            gateway: node.map(|n| n.create_gateway_with_timeout(connect_timeout, rpc_timout)),
            ack,
            sender,
        }
    }

    pub(crate) async fn deploy(self) -> Result<SubdataflowExecution, TaskDeploymentException> {
        let ack = self.ack;
        match &self.gateway {
            Some(gateway) => {
                let req = CreateSubDataflowRequest {
                    job_id: Some(self.subdataflow.get_job_id()),
                    dataflow: Some(self.subdataflow.clone()),
                };

                match gateway.create_sub_dataflow(req).await {
                    Ok(resp) => Ok(SubdataflowExecution::new(
                        Node::new(self.addr.clone()),
                        self.subdataflow,
                        self.execution_id.clone(),
                        resp.status(),
                        ack,
                        self.sender,
                    )),
                    Err(err) => Err(TaskDeploymentException::RpcError(err)),
                }
            }
            None => Err(TaskDeploymentException::InvalidWorkerEndpoint),
        }
    }
}

pub(crate) enum TaskDeploymentException {
    InvalidWorkerEndpoint,
    RpcError(tonic::Status),
}

/// A [`SubdataflowExecution`] represents a execution context of a subdataflow. It's responsible for:
/// - watch the status of subdataflow
/// - send heartbeat ack to TaskWorker
/// - store basic information of a subdataflow
/// - manage the checkpoint snapshot of a subdataflow
/// - manage the checkpoint trigger of a subdataflow
pub(crate) struct SubdataflowExecution {
    /// the remote TaskManager node
    worker: Node,
    /// all vertexes execution contexts
    vertexes: BTreeMap<ExecutorId, VertexExecution>,
    /// the id of the subdataflow execution
    execution_id: ExecutionID,
    /// the status of subdataflow
    status: DataflowStatus,
    /// the latest heartbeat ack id
    latest_ack_heartbeat_id: AtomicU64,
    /// the latest heartbeat timestamp
    latest_ack_heartbeat_timestamp: AtomicU64,
    /// the asynchronous task of the ack sender
    ack_handler: JoinHandle<()>,
    /// the enqueue-entrypoint of a ack request queue
    ack_request_queue: mpsc::Sender<Ack>,
}
impl SubdataflowExecution {
    pub(crate) fn new<G: 'static + ReceiveAckRpcGateway + Send + Sync>(
        worker: Node,
        subdataflow: Dataflow,
        execution_id: ExecutionID,
        status: DataflowStatus,
        ack: AckResponder<G>,
        ack_request_queue: mpsc::Sender<Ack>,
    ) -> Self {
        Self {
            worker,
            vertexes: subdataflow
                .nodes
                .iter()
                .map(|(executor_id, info)| (*executor_id, VertexExecution::new(executor_id, info)))
                .collect(),
            execution_id,
            status,
            latest_ack_heartbeat_id: AtomicU64::default(),
            latest_ack_heartbeat_timestamp: AtomicU64::default(),
            ack_handler: tokio::spawn(ack),
            ack_request_queue,
        }
    }

    pub(crate) fn try_terminate(&mut self) {
        todo!()
    }

    pub(crate) fn get_execution_id(&self) -> &ExecutionID {
        &self.execution_id
    }

    pub(crate) async fn update_heartbeat_status(&mut self, heartbeat: &Heartbeat) {
        match heartbeat.timestamp.as_ref() {
            Some(timestamp) => match heartbeat.node_type() {
                NodeType::TaskWorker => {
                    self.worker.update_status(NodeStatus::Running, timestamp);
                    let ref now = utils::times::now();
                    let _ = self
                        .ack_request_queue
                        .send(Ack {
                            timestamp: Some(from_utc_chrono_to_prost_timestamp(now)),
                            ack_type: AckType::Heartbeat as i32,
                            node_type: NodeType::JobManager as i32,
                            execution_id: Some(self.execution_id.into_prost()),
                            request_id: Some(RequestId::HeartbeatId(heartbeat.heartbeat_id)),
                        })
                        .await;
                }
                _ => {}
            },
            None => {}
        }
    }

    pub(crate) fn ack(&mut self, ack: &Ack) {
        match ack.ack_type() {
            AckType::Heartbeat => {
                if let Some(&RequestId::HeartbeatId(heartbeat_id)) = ack.request_id.as_ref() {
                    self.status = DataflowStatus::Running;
                    if self.latest_ack_heartbeat_id.load(atomic::Ordering::Relaxed) < heartbeat_id {
                        self.latest_ack_heartbeat_id
                            .swap(heartbeat_id, atomic::Ordering::AcqRel);
                        ack.timestamp.as_ref().iter().for_each(|timestamp| {
                            self.latest_ack_heartbeat_timestamp
                                .swap(timestamp.seconds as u64, atomic::Ordering::AcqRel);
                        })
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{self, AtomicU64};

    use common::{
        net::{
            cluster::{Node, NodeStatus},
            gateway::MockRpcGateway,
            AckResponderBuilder, PersistableHostAddr,
        },
        utils::times::prost_now,
    };
    use proto::common::{
        ack::{AckType, RequestId},
        Ack, DataflowStatus, ExecutionId, Heartbeat, NodeType,
    };

    #[tokio::test]
    async fn test_subdataflow_execution_update_heartbeat_status() {
        let ack_responder_builder = AckResponderBuilder {
            delay: 3,
            buf_size: 10,
            nodes: vec![PersistableHostAddr::default()],
        };

        let (gateway, mut ack_rx, _) = MockRpcGateway::new(ack_responder_builder.buf_size, 10);

        let (ack_responder, ack_tx) = ack_responder_builder.build(|_| gateway.clone());

        let mut execution = super::SubdataflowExecution {
            worker: Node::new(PersistableHostAddr::default()),
            vertexes: Default::default(),
            execution_id: Default::default(),
            status: DataflowStatus::Initialized,
            ack_handler: tokio::spawn(ack_responder),
            ack_request_queue: ack_tx,
            latest_ack_heartbeat_id: AtomicU64::default(),
            latest_ack_heartbeat_timestamp: AtomicU64::default(),
        };

        execution
            .update_heartbeat_status(&Heartbeat {
                heartbeat_id: 1,
                timestamp: Some(prost_now()),
                node_type: NodeType::TaskWorker as i32,
                execution_id: Some(ExecutionId {
                    job_id: Some(Default::default()),
                    sub_id: 0,
                }),
            })
            .await;

        assert_eq!(execution.worker.get_status(), &NodeStatus::Running);
        let option = ack_rx.recv().await;
        assert!(option.is_some());

        let result = option.unwrap();

        assert_eq!(result.request_id, Some(RequestId::HeartbeatId(1)));
        assert_eq!(result.ack_type(), AckType::Heartbeat);
        assert_eq!(result.node_type(), NodeType::JobManager);
        assert_eq!(
            result.execution_id,
            Some(ExecutionId {
                job_id: Some(Default::default()),
                sub_id: 0,
            })
        );
    }

    #[tokio::test]
    async fn test_subdataflow_execution_ack_heartbeat() {
        let ack_responder_builder = AckResponderBuilder {
            delay: 3,
            buf_size: 10,
            nodes: vec![PersistableHostAddr::default()],
        };

        let (gateway, _, _) = MockRpcGateway::new(ack_responder_builder.buf_size, 10);

        let (ack_responder, ack_tx) = ack_responder_builder.build(|_| gateway.clone());

        let mut execution = super::SubdataflowExecution {
            worker: Node::new(PersistableHostAddr::default()),
            vertexes: Default::default(),
            execution_id: Default::default(),
            status: DataflowStatus::Initialized,
            ack_handler: tokio::spawn(ack_responder),
            ack_request_queue: ack_tx,
            latest_ack_heartbeat_id: AtomicU64::default(),
            latest_ack_heartbeat_timestamp: AtomicU64::default(),
        };
        let now = prost_now();

        {
            execution.ack(&Ack {
                timestamp: Some(now.clone()),
                ack_type: AckType::Heartbeat as i32,
                node_type: NodeType::TaskWorker as i32,
                execution_id: Some(ExecutionId {
                    job_id: Default::default(),
                    sub_id: 1,
                }),
                request_id: Some(RequestId::HeartbeatId(2)),
            });

            assert_eq!(
                execution
                    .latest_ack_heartbeat_id
                    .load(atomic::Ordering::Relaxed),
                2
            );
            assert_eq!(
                execution
                    .latest_ack_heartbeat_timestamp
                    .load(atomic::Ordering::Relaxed),
                now.seconds as u64
            );
        }

        let now_1 = prost_now();
        {
            execution.ack(&Ack {
                timestamp: Some(now_1.clone()),
                ack_type: AckType::Heartbeat as i32,
                node_type: NodeType::TaskWorker as i32,
                execution_id: Some(ExecutionId {
                    job_id: Default::default(),
                    sub_id: 1,
                }),
                request_id: Some(RequestId::HeartbeatId(1)),
            });

            assert_eq!(
                execution
                    .latest_ack_heartbeat_id
                    .load(atomic::Ordering::Relaxed),
                2
            );
            assert_eq!(
                execution
                    .latest_ack_heartbeat_timestamp
                    .load(atomic::Ordering::Relaxed),
                now.seconds as u64
            );
        }
    }
}
