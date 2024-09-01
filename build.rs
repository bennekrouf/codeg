use dotenvy::from_path;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    // Get the generated folder from the environment variable
    let target_folder = env::var("TARGET_FOLDER").unwrap_or_else(|_| "generated".to_string());

    // Ensure the directory exists
    let generated_dir = PathBuf::from(target_folder);
    if !generated_dir.exists() {
        eprintln!("Error: The '{:?}' directory does not exist.", generated_dir);
        std::process::exit(1);
    }

    // Compile each .rs file in the generated directory
    for entry in fs::read_dir(&generated_dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            // Compile the file
            let output = Command::new("rustc")
                .arg(&path)
                .output()
                .expect("Failed to compile");

            if !output.status.success() {
                eprintln!(
                    "Compilation failed for {:?}:\n{}",
                    path,
                    String::from_utf8_lossy(&output.stderr)
                );
                std::process::exit(1);
            }
        }
    }
      // Get the OUT_DIR environment variable at runtime
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Construct the path to the descriptor set file
    let descriptor_path = out_dir.join("codeg_descriptor.bin");

    // Configure and compile the proto files
    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .compile(&["proto-definitions/codeg.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile codeg proto files: {}", e));  
}

