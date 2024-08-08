
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::models::Endpoint;

pub fn generate_endpoint(endpoint: &Endpoint, code_dir: &Path) -> std::io::Result<()> {
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

    Ok(())
}


