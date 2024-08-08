
use std::fs;
use std::process::Command;

fn main() {
    // Define the directory where generated files are located
    let dir = "generated";

    // Ensure the directory exists
    if !fs::metadata(dir).is_ok() {
        eprintln!("Error: The 'generated' directory does not exist.");
        std::process::exit(1);
    }

    // Compile each .rs file in the generated directory
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            // Compile the file
            let output = Command::new("rustc")
                .arg(path)
                .output()
                .expect("Failed to compile");

            if !output.status.success() {
                eprintln!("Compilation failed:\n{}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
    }
}

