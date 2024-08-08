
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate_main(generated_dir: &Path, file_stems: &[&str]) -> std::io::Result<()> {
    // Define the path for the `main.rs` file at the root of the generated folder
    let main_rs_path = generated_dir.join("main.rs");
    
    // Create or open the `main.rs` file
    let mut main_rs_file = fs::File::create(main_rs_path)?;
    
    // Generate the content for the `main.rs` file
    let mut main_rs_content = String::new();

    // Add module declarations for each file_stem
    for file_stem in file_stems {
        main_rs_content.push_str(&format!("mod {};\n", file_stem));
    }

    // Add the main function code
    main_rs_content.push_str(
        "\nuse std::io;\n\nfn main() -> io::Result<()> {\n    // Add your code here\n    println!(\"Hello from main!\");\n    Ok(())\n}\n"
    );

    // Write the content to the `main.rs` file
    main_rs_file.write_all(main_rs_content.as_bytes())?;

    Ok(())
}

