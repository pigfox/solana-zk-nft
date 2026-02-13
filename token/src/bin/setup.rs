use solana_zk_nft::zk::setup_keys;

fn main() {
    println!("🔐 ZK NFT Setup");
    println!("===============\n");

    if let Err(e) = setup_keys() {
        eprintln!("❌ Setup failed: {}", e);
        std::process::exit(1);
    }

    println!("\n✅ Setup complete! Keys generated:");
    println!("   - proving_key.bin");
    println!("   - verifying_key.bin");
}
