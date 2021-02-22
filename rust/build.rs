use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["../protobuf/requests.proto"], &["../protobuf"])?;
    Ok(())
}