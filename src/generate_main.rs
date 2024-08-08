
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate_main(generated_dir: &Path, file_stems: &[&str]) -> std::io::Result<()> {
    // Define the path for the `main.rs` file at the root of the generated folder
    let main_rs_path = generated_dir.join("main.rs");
    
    // Create or open the `main.rs` file
    let mut main_rs_file = fs::File::create(main_rs_path)?;
    
    // Generate the content for the `main.rs` file
    let mut main_rs_content = String::new();

    // Add module declarations for each file_stem
    for file_stem in file_stems {
        main_rs_content.push_str(&format!("mod {};\n", file_stem));
    }

    // Add the main function code with gRPC server setup
    main_rs_content.push_str(
        r#"
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
"#
    );

    // Add the endpoints to the gRPC service
    for file_stem in file_stems {
        main_rs_content.push_str(&format!("            String::from(\"{}\"),\n", file_stem));
    }

    main_rs_content.push_str(
        r#"
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
"#
    );

    // Write the content to the `main.rs` file
    main_rs_file.write_all(main_rs_content.as_bytes())?;

    Ok(())
}

