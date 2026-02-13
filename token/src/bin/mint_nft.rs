use anyhow::Result;
use ark_serialize::CanonicalSerialize;
use solana_sdk::signer::Signer;
use solana_zk_nft::{
    zk::{ZkProver, ZkVerifier},
    NftMinter,
};
use std::env;

fn main() -> Result<()> {
    println!("🎨 ZK NFT Minter");
    println!("================\n");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <secret_number>", args[0]);
        eprintln!("Example: {} 12345", args[0]);
        std::process::exit(1);
    }

    let secret: u64 = args[1].parse().expect("Secret must be a number");

    // Load ZK keys
    println!("🔑 Loading ZK keys...");
    let prover = ZkProver::from_file("proving_key.bin")?;
    let verifier = ZkVerifier::from_file("verifying_key.bin")?;

    // Generate proof
    println!("🔐 Generating ZK proof...");
    let (proof, commitment) = prover.generate_proof(secret)?;
    println!("✅ Proof generated");
    println!("   Commitment: {:?}", commitment);

    // Verify proof
    println!("\n🔍 Verifying proof...");
    let is_valid = verifier.verify(&proof, commitment)?;

    if !is_valid {
        eprintln!("❌ Invalid proof! Cannot mint NFT.");
        std::process::exit(1);
    }
    println!("✅ Proof verified!");

    // Save proof
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes)?;
    std::fs::write("proof.bin", &proof_bytes)?;
    println!("💾 Proof saved to proof.bin");

    // Mint NFT
    println!("\n🪙 Minting NFT...");
    let minter = NftMinter::new("https://api.devnet.solana.com")?;

    let balance = minter.check_balance()?;
    println!("💰 Balance: {} SOL", balance);

    if balance < 0.1 {
        eprintln!("⚠️  Low balance! Run: solana airdrop 2");
        std::process::exit(1);
    }

    let mint =
        minter.mint_nft("https://ipfs.io/ipfs/QmPhPb2Cp59ZfbmP2ZuoANghBTxF7KiBFJLZMZAES4evC2")?;

    println!("\n🎉 NFT Minted Successfully!");
    println!("\n📊 Details:");
    println!("   Mint: {}", mint);
    println!("   Owner: {}", minter.payer.pubkey());
    println!("\n🔍 View on Explorer:");
    println!(
        "   https://explorer.solana.com/address/{}?cluster=devnet",
        mint
    );

    Ok(())
}
