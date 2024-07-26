use proto::agent_server::{Agent, AgentServer};
mod proto {
    tonic::include_proto!("registration");
}

#[derive(Debug, Default)]
struct RegistrationService {}
#[tonic::async_trait]
impl Agent for RegistrationService {
    async fn register_agent(
        &self,
        request: tonic::Request<proto::Health>,
    ) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
    }
}
fn main() {
    println!("Hello, world!");
}
