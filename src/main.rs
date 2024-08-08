
use std::fs;
use std::io::Write;
use serde_yaml;
use models::{Endpoint, Entity};
use std::env;

mod models;

// Generate code and write to files
fn generate_code_for_endpoints(endpoints: &[Endpoint]) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;

    // Define the output directories
    let code_dir = current_dir.join("generated/endpoints");
    let proto_dir = current_dir.join("generated/proto");

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
        let function_name = endpoint.path.replace("-", "_");

        // Generate Rust file
        let rust_file_name = code_dir.join(format!("{}.rs", function_name));
        let mut rust_file = fs::File::create(rust_file_name)?;
        let rust_content = format!(
            "pub fn {}() {{\n    println!(\"{} function not implemented.\");\n}}\n",
            function_name,
            endpoint.path
        );
        rust_file.write_all(rust_content.as_bytes())?;

        // Add `pub mod` statement for the generated file to `mod.rs`
        mod_rs_content.push_str(&format!("pub mod {};\n", function_name));

        // Generate .proto file
        let proto_file_name = proto_dir.join(format!("{}.proto", function_name));
        let mut proto_file = fs::File::create(proto_file_name)?;
        let proto_content = format!(
            "syntax = \"proto3\";\n\nservice {} {{\n    rpc {} (Empty) returns (Empty);\n}}\n\nmessage Empty {{}}\n",
            function_name, function_name
        );
        proto_file.write_all(proto_content.as_bytes())?;
    }

    // Write the `mod.rs` file
    let mod_rs_path = code_dir.join("mod.rs");
    let mut mod_rs_file = fs::File::create(mod_rs_path)?;
    mod_rs_file.write_all(mod_rs_content.as_bytes())?;

    // Generate the basic `main.rs` file
    let main_rs_path = code_dir.join("main.rs");
    let mut main_rs_file = fs::File::create(main_rs_path)?;
    let main_rs_content = format!(
        "use std::io;\n\nfn main() -> io::Result<()> {{\n    // Add your code here\n    println!(\"Hello from main!\");\n    Ok(())\n}}\n"
    );
    main_rs_file.write_all(main_rs_content.as_bytes())?;

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

    // Generate code files and .proto files for each endpoint
    match generate_code_for_endpoints(&entity.endpoints) {
        Ok(()) => println!("Code and .proto generation complete."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

