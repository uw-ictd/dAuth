use tonic::{transport::Server, Request, Response, Status};

use example_proto::example_call_server::{ExampleCall, ExampleCallServer};
use example_proto::{HelloReq, HelloResp};

pub mod example_proto {
    tonic::include_proto!("example_proto");
}

#[derive(Default)]
pub struct MyExample {}

#[tonic::async_trait]
impl ExampleCall for MyExample {
    async fn say_hello(
        &self,
        request: Request<HelloReq>,
    ) -> Result<Response<HelloResp>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = example_proto::HelloResp {
            response: format!("Hello {:?}!", request.into_inner().message).into_bytes(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyExample::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ExampleCallServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
