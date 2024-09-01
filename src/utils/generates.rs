
use std::{env, fs};
use std::io::Write;
use dotenv::dotenv;
use std::path::PathBuf;
use tracing::{info, error};

use crate::utils::generate_cargo_toml::generate_cargo_toml;
use crate::utils::generate_endpoint::generate_endpoint;
use crate::utils::generate_main::generate_main;
use crate::utils::generate_proto::generate_proto;
use crate::models::Endpoint;

pub fn generates(endpoints: &[Endpoint], file_stem: &str) -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv().ok();
    info!("Environment variables loaded from .env file.");

    // Get the generated folder from the environment variable
    let generated_folder = env::var("GENERATED_FOLDER").unwrap_or_else(|_| {
        error!("'GENERATED_FOLDER' environment variable not set. Using default 'generated' folder.");
        "generated".to_string()
    });
    info!("Using '{}' as the generated folder.", generated_folder);

    // Define the base output directory from the environment variable
    let generated_dir = PathBuf::from(&generated_folder);
    let generated_src_dir = generated_dir.join("src");
    info!("Generated source directory: {:?}", generated_src_dir);

    // Define the output directories, incorporating file_stem within the src subdirectory
    let file_stem_dir = generated_src_dir.join(file_stem);
    let code_dir = file_stem_dir.join("endpoints");
    let proto_dir = file_stem_dir.join("proto");

    // Ensure the directories exist
    if !code_dir.exists() {
        fs::create_dir_all(&code_dir)?;
        info!("Created code directory: {:?}", code_dir);
    } else {
        info!("Code directory already exists: {:?}", code_dir);
    }

    if !proto_dir.exists() {
        fs::create_dir_all(&proto_dir)?;
        info!("Created proto directory: {:?}", proto_dir);
    } else {
        info!("Proto directory already exists: {:?}", proto_dir);
    }

    // Generate files for each endpoint
    let mut mod_rs_content = String::new();
    for endpoint in endpoints {
        info!("Generating files for endpoint: {}", endpoint.path);

        // Generate endpoint code and proto files
        if let Err(e) = generate_endpoint(endpoint, &code_dir) {
            error!("Failed to generate endpoint for {}: {:?}", endpoint.path, e);
            return Err(e);
        }
        info!("Generated endpoint code for {}.", endpoint.path);

        if let Err(e) = generate_proto(endpoint, &proto_dir) {
            error!("Failed to generate proto for {}: {:?}", endpoint.path, e);
            return Err(e);
        }
        info!("Generated proto file for {}.", endpoint.path);

        // Add `pub mod` statement for the generated file to `mod.rs`
        mod_rs_content.push_str(&format!("pub mod {};\n", endpoint.path.replace("-", "_")));
    }

    // Write the `mod.rs` file for the specific file_stem in the `endpoints` directory
    let mod_rs_path = code_dir.join("mod.rs");
    let mut mod_rs_file = fs::File::create(&mod_rs_path)?;
    mod_rs_file.write_all(mod_rs_content.as_bytes())?;
    info!("Written mod.rs file at {:?}", mod_rs_path);

    // Generate a `mod.rs` file in the `generated/src/[file_stem]` directory that contains "mod endpoints;"
    let file_stem_mod_rs_path = file_stem_dir.join("mod.rs");
    let mut file_stem_mod_rs_file = fs::File::create(&file_stem_mod_rs_path)?;
    file_stem_mod_rs_file.write_all(b"mod endpoints;\n")?;
    info!("Written file_stem mod.rs file at {:?}", file_stem_mod_rs_path);

    // Collect all file_stems for later use in generating the main.rs file
    let file_stems = vec![file_stem];

    // Generate the main.rs file in the `src` subdirectory
    if let Err(e) = generate_main(&generated_src_dir, &file_stems) {
        error!("Failed to generate main.rs: {:?}", e);
        return Err(e);
    }
    info!("Generated main.rs in {:?}", generated_src_dir);

    // Generate the Cargo.toml file if it doesn't already exist
    let cargo_toml_path = generated_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        if let Err(e) = generate_cargo_toml(file_stem, &generated_dir) {
            error!("Failed to generate Cargo.toml: {:?}", e);
            return Err(e);
        }
        info!("Generated Cargo.toml at {:?}", cargo_toml_path);
    } else {
        info!("Cargo.toml already exists at {:?}", cargo_toml_path);
    }

    info!("File generation for '{}' completed successfully.", file_stem);
    Ok(())
}

