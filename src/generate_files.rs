use std::fs;
use std::ffi::OsStr;
use crate::generates::generates;
use serde_yaml;
use crate::models::Entity;
use std::env;


pub fn generate_files() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the generation folder from environment variables
    let config_dir = env::var("TARGET_FOLDER").expect("Missing 'TARGET_FOLDER' environment variable");

    // Iterate over each YAML file in the configuration directory
    for entry in fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the file has a .yml extension
        if path.extension() == Some(OsStr::new("yml")) {
            // Extract the file name without extension
            let file_stem = path.file_stem()
                .ok_or("Failed to get file stem")?
                .to_str()
                .ok_or("Failed to convert file stem to str")?;

            // Read the YAML file
            let yaml_content = fs::read_to_string(&path)?;

            // Deserialize the YAML content into the Entity struct
            let entity: Entity = serde_yaml::from_str(&yaml_content)?;

            // Generate code files and .proto files for each endpoint
            generates(&entity.endpoints, file_stem)?;
            println!("Code and .proto generation complete for {}.", file_stem);
        }
    }

    Ok(())
}

