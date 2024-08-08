
use std::fs;
use std::ffi::OsStr;
use generates::generates;
use serde_yaml;
use models::Entity;

mod models;
mod generates;
mod generate_proto;
mod generate_main;
mod generate_endpoint;

fn main() {
    // Define the configuration directory
    let config_dir = "configuration";

    // Iterate over each YAML file in the configuration directory
    for entry in fs::read_dir(config_dir).expect("Failed to read configuration directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        // Check if the file has a .yml extension
        if path.extension() == Some(OsStr::new("yml")) {
            // Extract the file name without extension
            let file_stem = path.file_stem()
                .expect("Failed to get file stem")
                .to_str()
                .expect("Failed to convert file stem to str");

            // Read the YAML file
            let yaml_content = fs::read_to_string(&path)
                .expect("Failed to read YAML file");

            // Deserialize the YAML content into the Entity struct
            let entity: Entity = serde_yaml::from_str(&yaml_content)
                .expect("Failed to deserialize YAML content");

            // Generate code files and .proto files for each endpoint
            match generates(&entity.endpoints, file_stem) {
                Ok(()) => println!("Code and .proto generation complete for {}.", file_stem),
                Err(e) => eprintln!("Error generating code for {}: {}", file_stem, e),
            }
        }
    }
}

