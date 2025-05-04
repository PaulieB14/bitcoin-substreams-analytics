use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .btree_map(["bitcoin.analytics.v1.BlockStats"])
        .btree_map(["bitcoin.analytics.v1.TransactionStats"])
        .btree_map(["bitcoin.analytics.v1.AddressStats"])
        .btree_map(["bitcoin.analytics.v1.UTXOStats"])
        .btree_map(["bitcoin.analytics.v1.MempoolData"])
        .include_file("mod.rs")
        .compile_protos(&["proto/analytics.proto"], &["proto/"])?;
    Ok(())
}
