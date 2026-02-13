use anyhow::Result;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use solana_zk_nft::NftMinter;
use std::{env, str::FromStr};

fn main() -> Result<()> {
    println!("📤 NFT Transfer");
    println!("===============\n");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <mint_address> <recipient_address>", args[0]);
        std::process::exit(1);
    }

    let mint = Pubkey::from_str(&args[1])?;
    let recipient = Pubkey::from_str(&args[2])?;

    println!("🪙 Mint: {}", mint);
    println!("👤 Recipient: {}", recipient);

    let minter = NftMinter::new("https://api.devnet.solana.com")?;

    println!("📦 Transferring from: {}", minter.payer.pubkey());
    println!("\n⏳ Processing transfer...");
    minter.transfer_nft(&mint, &recipient)?;

    println!("✅ Transfer complete!");
    println!("\n🔍 View on Explorer:");
    println!(
        "   https://explorer.solana.com/address/{}?cluster=devnet",
        mint
    );

    Ok(())
}
