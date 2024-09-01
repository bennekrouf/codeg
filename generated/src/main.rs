mod orders;

use std::io;
use tonic::{transport::Server, Request, Response, Status};
// use prost::Message;
// use tokio::sync::Mutex;
// use std::sync::Arc;

pub mod configuration_service {
    tonic::include_proto!("configuration_service");
}

use configuration_service::configuration_service_server::{ConfigurationService, ConfigurationServiceServer};
use configuration_service::{EndpointList, Empty};

#[derive(Debug, Default)]
pub struct MyConfigurationService {
    endpoints: Vec<String>,
}

#[tonic::async_trait]
impl ConfigurationService for MyConfigurationService {
    async fn get_endpoints(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<EndpointList>, Status> {
        let endpoints = self.endpoints.clone();
        let reply = EndpointList { endpoints };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Add your code here

    let addr = "[::1]:50051".parse().unwrap();
    let service = MyConfigurationService {
        endpoints: vec![
            String::from("orders"),

        ],
    };

    let svc = ConfigurationServiceServer::new(service);

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
