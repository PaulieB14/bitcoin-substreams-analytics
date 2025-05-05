use std::io::Result;

fn main() -> Result<()> {
    // Generate Rust code from the protobuf definitions
    prost_build::compile_protos(&["proto/analytics.proto", "proto/utxo.proto"], &["proto/"])?;
    Ok(())
}
