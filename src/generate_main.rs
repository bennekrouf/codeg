
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate_main(code_dir: &Path) -> std::io::Result<()> {
    // Generate the basic `main.rs` file
    let main_rs_path = code_dir.join("main.rs");
    let mut main_rs_file = fs::File::create(main_rs_path)?;
    let main_rs_content = format!(
        "use std::io;\n\nfn main() -> io::Result<()> {{\n    // Add your code here\n    println!(\"Hello from main!\");\n    Ok(())\n}}\n"
    );
    main_rs_file.write_all(main_rs_content.as_bytes())?;

    Ok(())
}


