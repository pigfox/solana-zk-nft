#!/bin/bash
clear
echo "🚀 ZK NFT Complete Demo"
echo "======================="
echo ""

# Build
echo "🔨 Building..."
cargo build --release
echo ""

# Setup keys if they don't exist
if [ ! -f "proving_key.bin" ] || [ ! -f "verifying_key.bin" ]; then
    echo "🔧 Setting up ZK keys (this may take a minute)..."
    cargo run --release --bin setup
    echo ""
else
    echo "✅ ZK keys already exist, skipping setup"
    echo ""
fi

# Run tests
echo "🧪 Running tests..."
cargo test --release 2>&1 | grep -E "(test result|running|ok|FAILED)" || cargo test --release
echo ""

# Solana setup
echo "📋 Configuring Solana..."
solana config set --url devnet
echo ""

# Check balance and request airdrop if needed
BALANCE=$(solana balance 2>/dev/null | awk '{print $1}' || echo "0")
echo "💰 Current balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 0.5" | bc -l 2>/dev/null || echo "1") )); then
    echo "⚠️  Low balance! Requesting airdrop..."
    if solana airdrop 2 2>/dev/null; then
        echo "✅ Airdrop successful"
    else
        echo "⚠️  Airdrop failed (rate limit reached)"
        echo "   Visit https://faucet.solana.com for manual airdrop"
        echo "   Or wait a few minutes and try again"
        read -p "   Press Enter to continue anyway or Ctrl+C to exit..."
    fi
    echo ""
fi

# Mint NFT with ZK proof
echo "🎨 Minting NFT with ZK proof (secret: 12345)..."
OUTPUT=$(cargo run --release --bin mint-nft 12345 2>&1)
echo "$OUTPUT"
echo ""

# Extract mint address from output
MINT_ADDRESS=$(echo "$OUTPUT" | grep -oP 'Mint: \K[A-Za-z0-9]+' | head -1)

if [ -n "$MINT_ADDRESS" ]; then
    echo "✅ NFT Successfully Minted!"
    echo "   Mint Address: $MINT_ADDRESS"
    echo "   Explorer: https://explorer.solana.com/address/$MINT_ADDRESS?cluster=devnet"
    echo ""
    echo "📝 To transfer this NFT, run:"
    echo "   cargo run --release --bin transfer-nft $MINT_ADDRESS RECIPIENT_ADDRESS"
    echo ""
    echo "   Example:"
    echo "   cargo run --release --bin transfer-nft $MINT_ADDRESS 2Kc5pYxgjnUZZUwvmyJpcXGf4HYUc5qQupPeapnLE1B2"
else
    echo "❌ Failed to extract mint address"
fi

echo ""
echo "🎉 Demo complete!"
