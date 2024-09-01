pub mod generated {
    tonic::include_proto!("codeg"); // The proto package name
}

mod models;
mod generate_files;
mod utils;

use std::sync::Arc;
use tokio::sync::Mutex;
use messengerc::{connect_to_messenger_service, MessagingService};
use generated::code_generator_server::{CodeGenerator, CodeGeneratorServer};
use generated::{GenerateFilesRequest, GenerateFilesResponse};
use tracing::error;
use std::path::Path;
use dotenvy::from_path;
use std::env;

use tonic::{transport::Server, Request, Response, Status};
use crate::generate_files::generate_files;

#[derive(Debug, Default)]
pub struct MyCodeGenerator;

#[tonic::async_trait]
impl CodeGenerator for MyCodeGenerator {
    async fn generate_files(
        &self,
        request: Request<GenerateFilesRequest>,
    ) -> Result<Response<GenerateFilesResponse>, Status> {
        // Extract the tenant from the request
        let tenant = request.into_inner().tenant;

        // Check if the tenant field is provided
        if tenant.is_empty() {
            // Log an error when the tenant is missing
            error!("Attempt to generate files without providing a tenant.");

            return Ok(Response::new(GenerateFilesResponse {
                message: "Tenant is required".into(),
                success: false,
            }));
        }

        // Call the `generate_files` function with the tenant as an argument
        match generate_files(&tenant) {
            Ok(_) => Ok(Response::new(GenerateFilesResponse {
                message: "File generation successful.".into(),
                success: true,
            })),
            Err(e) => {
                error!("File generation failed for tenant {}: {:?}", tenant, e);
                Ok(Response::new(GenerateFilesResponse {
                    message: format!("File generation failed: {}", e),
                    success: false,
                }))
            }
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

