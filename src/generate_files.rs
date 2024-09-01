
use std::{env, fs};
use std::ffi::OsStr;
use crate::utils::generates::generates;
use serde_yaml;
use crate::models::Entity;
use tracing::{info, error, warn};

pub fn generate_files() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the generation folder from environment variables
    let config_dir = match env::var("YML_FOLDER") {
        Ok(dir) => {
            info!("Using '{}' as the target configuration directory.", dir);
            dir
        }
        Err(e) => {
            error!("Missing 'YML_FOLDER' environment variable: {:?}", e);
            return Err(Box::new(e));
        }
    };

    // Track if any YAML files are processed
    let mut yaml_files_found = false;

    // Iterate over each file in the configuration directory
    for entry in fs::read_dir(&config_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the file has a .yml extension
        if path.extension() == Some(OsStr::new("yml")) {
            yaml_files_found = true;
            info!("Processing YAML file: {:?}", path);

            // Extract the file name without extension
            let file_stem = match path.file_stem().and_then(|os_str| os_str.to_str()) {
                Some(stem) => stem,
                None => {
                    error!("Failed to get or convert file stem for file: {:?}", path);
                    continue;
                }
            };

            // Read the YAML file
            let yaml_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(e) => {
                    error!("Failed to read YAML file: {:?}, error: {:?}", path, e);
                    continue;
                }
            };

            // Deserialize the YAML content into the Entity struct
            let entity: Entity = match serde_yaml::from_str(&yaml_content) {
                Ok(entity) => entity,
                Err(e) => {
                    error!("Failed to deserialize YAML content from file {:?}: {:?}", path, e);
                    continue;
                }
            };

            // Generate code files and .proto files for each endpoint
            match generates(&entity.endpoints, file_stem) {
                Ok(()) => info!("Code and .proto generation complete for {}.", file_stem),
                Err(e) => error!("Error generating code for {}: {:?}", file_stem, e),
            }
        } else {
            info!("Skipping non-YAML file: {:?}", path);
        }
    }

    // Alert if no YAML files were found
    if !yaml_files_found {
        warn!("No YAML files found in the configuration directory: '{}'", config_dir);
    } else {
        info!("File generation completed successfully.");
    }

    Ok(())
}

