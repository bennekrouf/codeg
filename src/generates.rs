
use std::{env, fs};
use std::io::Write;

use crate::generate_endpoint::generate_endpoint;
use crate::generate_main::generate_main;
use crate::generate_proto::generate_proto;
use crate::models::Endpoint;

pub fn generates(endpoints: &[Endpoint], file_stem: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;

    // Define the output directories, incorporating file_stem
    let code_dir = current_dir.join(format!("generated/{}/endpoints", file_stem));
    let proto_dir = current_dir.join(format!("generated/{}/proto", file_stem));
    let generated_dir = current_dir.join("generated");

    // Ensure the directories exist
    if !code_dir.exists() {
        fs::create_dir_all(&code_dir)?;
    }
    if !proto_dir.exists() {
        fs::create_dir_all(&proto_dir)?;
    }

    // Generate files for each endpoint
    let mut mod_rs_content = String::new();
    for endpoint in endpoints {
        generate_endpoint(endpoint, &code_dir)?;
        generate_proto(endpoint, &proto_dir)?;

        // Add `pub mod` statement for the generated file to `mod.rs`
        mod_rs_content.push_str(&format!("pub mod {};\n", endpoint.path.replace("-", "_")));
    }

    // Write the `mod.rs` file for the specific file_stem
    let mod_rs_path = code_dir.join("mod.rs");
    let mut mod_rs_file = fs::File::create(mod_rs_path)?;
    mod_rs_file.write_all(mod_rs_content.as_bytes())?;

    // Collect all file_stems for later use in generating the main.rs file
    let file_stems = vec![file_stem];

    // Generate the main.rs file at the root of the generated directory
    generate_main(&generated_dir, &file_stems)?;

    Ok(())
}

