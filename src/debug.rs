use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::{Block, Transaction};

pub fn print_block_structure(block: &Block) {
    println!("Block fields:");
    println!("  hash: {:?}", block.hash);
    println!("  size: {}", block.size);
    println!("  stripped_size: {}", block.stripped_size);
    println!("  weight: {}", block.weight);
    println!("  height: {}", block.height);
    println!("  version: {}", block.version);
    println!("  merkle_root: {:?}", block.merkle_root);
    println!("  time: {}", block.time);
    println!("  median_time: {}", block.median_time);
    println!("  nonce: {}", block.nonce);
    println!("  bits: {}", block.bits);
    println!("  difficulty: {}", block.difficulty);
    println!("  previous_block_hash: {:?}", block.previous_block_hash);
    println!("  chain_work: {:?}", block.chain_work);
    println!("  coinbase_param: {:?}", block.coinbase_param);
    println!("  transaction_count: {}", block.transaction_count);
    
    if !block.transactions.is_empty() {
        println!("\nFirst Transaction fields:");
        let tx = &block.transactions[0];
        print_transaction_structure(tx);
    }
}

pub fn print_transaction_structure(tx: &Transaction) {
    println!("Transaction fields:");
    println!("  hex: {}", tx.hex);
    println!("  txid: {}", tx.txid);
    println!("  hash: {}", tx.hash);
    println!("  size: {}", tx.size);
    println!("  vsize: {}", tx.vsize);
    println!("  weight: {}", tx.weight);
    println!("  version: {}", tx.version);
    println!("  locktime: {}", tx.locktime);
    println!("  vin_count: {}", tx.vin_count);
    println!("  vout_count: {}", tx.vout_count);
    
    println!("\n  Inputs (vin):");
    for (i, vin) in tx.vin.iter().enumerate() {
        println!("    Input {}:", i);
        println!("      txid: {}", vin.txid);
        println!("      vout: {}", vin.vout);
        println!("      script_sig_asm: {}", vin.script_sig_asm);
        println!("      script_sig_hex: {}", vin.script_sig_hex);
        println!("      sequence: {}", vin.sequence);
        println!("      coinbase: {}", vin.coinbase);
        println!("      txinwitness: {:?}", vin.txinwitness);
    }
    
    println!("\n  Outputs (vout):");
    for (i, vout) in tx.vout.iter().enumerate() {
        println!("    Output {}:", i);
        println!("      value: {}", vout.value);
        println!("      n: {}", vout.n);
        println!("      script_pub_key_asm: {}", vout.script_pub_key_asm);
        println!("      script_pub_key_hex: {}", vout.script_pub_key_hex);
        println!("      script_pub_key_type: {}", vout.script_pub_key_type);
        println!("      script_pub_key_address: {}", vout.script_pub_key_address);
    }
}
