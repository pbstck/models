use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    prost_build::compile_protos(&["../protobuf/requests.proto"], &["../protobuf"])?;

    let generated_file = Path::new(env::var("OUT_DIR").unwrap().as_str()).join("model.rs");
    let proto_out = Path::new("src/lib.rs");
    match fs::remove_file(&proto_out) {
        Err(error) => println!("Error removing file {:?}", error),
        Ok(()) => (),
    };

    fs::copy(generated_file, proto_out)?;
    Ok(())
}