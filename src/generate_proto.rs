
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::models::Endpoint;

pub fn generate_proto(endpoint: &Endpoint, proto_dir: &Path) -> std::io::Result<()> {
    let function_name = endpoint.path.replace("-", "_");

    // Generate .proto file
    let proto_file_name = proto_dir.join(format!("{}.proto", function_name));
    let mut proto_file = fs::File::create(proto_file_name)?;
    let proto_content = format!(
        "syntax = \"proto3\";\n\nservice {} {{\n    rpc {} (Empty) returns (Empty);\n}}\n\nmessage Empty {{}}\n",
        function_name, function_name
    );
    proto_file.write_all(proto_content.as_bytes())?;

    Ok(())
}


