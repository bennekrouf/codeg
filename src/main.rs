
use std::fs;
use generates::generates;
use serde_yaml;
use models::Entity;

mod models;
mod generates;
mod generate_proto;
mod generate_main;
mod generate_endpoint;

fn main() {
    // Read the YAML file
    let yaml_file_path = "orders.yml";
    let yaml_content = fs::read_to_string(yaml_file_path)
        .expect("Failed to read YAML file");

    // Deserialize the YAML content into the Entity struct
    let entity: Entity = serde_yaml::from_str(&yaml_content)
        .expect("Failed to deserialize YAML content");

    // Generate code files and .proto files for each endpoint
    match generates(&entity.endpoints) {
        Ok(()) => println!("Code and .proto generation complete."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

