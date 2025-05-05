use std::io::Result;
use substreams_protogen::Protogen;

fn main() -> Result<()> {
    // Generate Rust code from the protobuf definitions
    Protogen::new()
        .with_input_path("proto/analytics.proto")
        .with_output_path("src/pb/bitcoin")
        .run_from_script()
}
