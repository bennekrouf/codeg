pub mod generated {
    tonic::include_proto!("codeg"); // The proto package name
}

mod models;
mod generate_files;
mod utils;

use tokio::sync::Mutex;
use messengerc::{connect_to_messenger_service, MessagingService};
use generated::code_generator_server::{CodeGenerator, CodeGeneratorServer};
use generated::{GenerateFilesRequest, GenerateFilesResponse};
// use tracing::info;
use std::sync::Arc;
use std::path::Path;
use dotenvy::from_path;
use std::env;

// use generate_files::generate_files;
use tonic::{transport::Server, Request, Response, Status};
use crate::generate_files::generate_files;

#[derive(Debug, Default)]
pub struct MyCodeGenerator;

#[tonic::async_trait]
impl CodeGenerator for MyCodeGenerator {
    async fn generate_files(
        &self,
        _request: Request<GenerateFilesRequest>,
    ) -> Result<Response<GenerateFilesResponse>, Status> {
        // Call the `generate_files` function and handle any errors.
        match generate_files() {
            Ok(_) => Ok(Response::new(GenerateFilesResponse {
                message: "File generation successful.".into(),
                success: true,
            })),
            Err(e) => Ok(Response::new(GenerateFilesResponse {
                message: format!("File generation failed: {}", e),
                success: false,
            })),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    // Retrieve the necessary values from environment variables
    let ip = env::var("CODEG_DOMAIN").expect("Missing 'domain' environment variable");
    let port = env::var("CODEG_PORT").expect("Missing 'port' environment variable");
    let addr = format!("{}:{}", ip, port).parse()?;

    let tag = env::var("CODEG_TAG").expect("Missing 'tag' environment variable");

    // Create and initialize the gRPC client for the messaging service
    let messenger_client = connect_to_messenger_service().await
        .ok_or("Failed to connect to messenger service")?;

    let messaging_service = MessagingService::new(
        Arc::new(Mutex::new(messenger_client)),
        tag.clone(),
    );

    let mes = format!("Codeg listening on {}", &addr);
    let _ = messaging_service.publish_message(mes.to_string(), Some(vec![tag])).await;

    let code_generator = MyCodeGenerator::default();

    // Start the gRPC server
    Server::builder()
        .add_service(CodeGeneratorServer::new(code_generator))
        .serve(addr)
        .await?;

    Ok(())
}

