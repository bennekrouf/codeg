use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate_cargo_toml(file_stem: &str, file_stem_dir: &Path) -> std::io::Result<()> {
    // Define the path for the `Cargo.toml` file
    let cargo_toml_path = file_stem_dir.join("Cargo.toml");

    // Create or open the `Cargo.toml` file
    let mut cargo_toml_file = fs::File::create(cargo_toml_path)?;

    // Generate the content for the `Cargo.toml` file
    let cargo_toml_content = format!(
        r#"[package]
name = "{file_stem}"
version = "0.1.0"
edition = "2021"

[dependencies]
tonic = "0.8"
tonic-build = "0.8"
prost = "0.11"
tokio = {{ version = "1", features = ["full"] }}

[build-dependencies]
tonic-build = "0.8"
"#,
        file_stem = file_stem
    );

    // Write the content to the `Cargo.toml` file
    cargo_toml_file.write_all(cargo_toml_content.as_bytes())?;

    Ok(())
}

