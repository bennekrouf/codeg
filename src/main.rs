
use std::fs;
use std::io::Write;
use serde_yaml;
use models::{Endpoint, Entity};
use std::env;

mod models;
mod generated;

// Generate code and write to files
fn generate_code_for_endpoints(endpoints: &[Endpoint]) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    
    // Define the output directory inside src
    let dir = current_dir.join("src/generated");
    
    // Ensure the directory exists
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    // Generate files for each endpoint
    let mut mod_rs_content = String::new();
    for endpoint in endpoints {
        let function_name = endpoint.path.replace("-", "_");
        let file_name = dir.join(format!("{}.rs", function_name));
        let mut file = fs::File::create(file_name)?;
        let content = format!(
            "pub fn {}() {{\n    println!(\"{} function not implemented.\");\n}}\n",
            function_name,
            endpoint.path
        );
        file.write_all(content.as_bytes())?;

        // Add `pub mod` statement for the generated file to `mod.rs`
        mod_rs_content.push_str(&format!("pub mod {};\n", function_name));
    }

    // Write the `mod.rs` file
    let mod_rs_path = dir.join("mod.rs");
    let mut mod_rs_file = fs::File::create(mod_rs_path)?;
    mod_rs_file.write_all(mod_rs_content.as_bytes())?;

    Ok(())
}

fn main() {
    // Read the YAML file
    let yaml_file_path = "orders.yml";
    let yaml_content = fs::read_to_string(yaml_file_path)
        .expect("Failed to read YAML file");

    // Deserialize the YAML content into the Entity struct
    let entity: Entity = serde_yaml::from_str(&yaml_content)
        .expect("Failed to deserialize YAML content");

    // Generate code files for each endpoint
    match generate_code_for_endpoints(&entity.endpoints) {
        Ok(()) => println!("Code generation complete."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

