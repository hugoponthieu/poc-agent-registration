use std::error::Error;

mod proto {
    tonic::include_proto!("registration");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:5001";
    let mut client = proto::agent_client::AgentClient::connect(url).await?;
    let req = proto::Health {
        cpu_usage: 1,
        memory_usage: 1,
    };
    let request = tonic::Request::new(req);
    let response = client.register_agent(request).await?;
    println!("Response: {:?}", response.get_ref().id);
    Ok(())
}
