use super::coord;
use proto::common::{
    Ack, Dataflow, DataflowStates, DataflowStatus, Heartbeat, ResourceId, Response,
};

use proto::coordinator::coordinator_api_server::CoordinatorApi;
use proto::coordinator::GetDataflowRequest;

use tonic::async_trait;

pub struct CoordinatorApiImpl {
    coordinator: coord::Coordinator,
}

impl CoordinatorApiImpl {
    pub(crate) fn new(coordinator: coord::Coordinator) -> CoordinatorApiImpl {
        CoordinatorApiImpl { coordinator }
    }
}

unsafe impl Send for CoordinatorApiImpl {}

unsafe impl Sync for CoordinatorApiImpl {}

#[async_trait]
impl CoordinatorApi for CoordinatorApiImpl {
    async fn receive_heartbeat(
        &self,
        request: tonic::Request<Heartbeat>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        self.coordinator.receive_heartbeart(request.get_ref()).await;
        Ok(tonic::Response::new(Response::ok()))
    }

    async fn receive_ack(
        &self,
        request: tonic::Request<Ack>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        self.coordinator.receive_ack(request.into_inner()).await;
        Ok(tonic::Response::new(Response::ok()))
    }

    async fn create_dataflow(
        &self,
        request: tonic::Request<Dataflow>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        self.coordinator
            .create_dataflow(request.into_inner())
            .await
            .map(|_| tonic::Response::new(Response::ok()))
    }
    async fn terminate_dataflow(
        &self,
        request: tonic::Request<ResourceId>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        self.coordinator
            .terminate_dataflow(request.get_ref())
            .await
            .map(|status| tonic::Response::new(Response::ok()))
    }
    async fn get_dataflow(
        &self,
        request: tonic::Request<GetDataflowRequest>,
    ) -> Result<tonic::Response<DataflowStates>, tonic::Status> {
        match self
            .coordinator
            .get_dataflow(request.get_ref().job_id.as_ref().unwrap())
        {
            Some(resp) => Ok(tonic::Response::new(DataflowStates {
                graph: Some(resp),
                task_infos: vec![],
                status: DataflowStatus::Running as i32,
            })),
            None => Err(tonic::Status::not_found(format!(
                "dataflow {:?} does not found",
                request.get_ref().job_id.as_ref().unwrap()
            ))),
        }
    }
}
