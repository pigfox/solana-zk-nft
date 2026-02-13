# ZK Proof NFT Minter

A complete Solana NFT minting system with zero-knowledge proof authentication.

## Features

✅ Zero-knowledge proof generation and verification (Groth16)  
✅ Solana NFT minting with Metaplex  
✅ NFT transfer functionality  
✅ Comprehensive tests  
✅ Pure Rust implementation (no JavaScript!)  

## Quick Start

### 1. Prerequisites
```bash
# Ensure Solana CLI is installed and configured
solana --version
solana config set --url devnet

# Get some SOL for testing
solana airdrop 2
```

### 2. Run Everything
```bash
./run.sh
```

This will:
- Generate ZK proving/verifying keys
- Run all tests
- Mint an NFT with ZK proof verification
- Display the NFT on Solana Explorer

### 3. Manual Steps

**Setup ZK keys (one time):**
```bash
cargo run --release --bin setup
```

**Run tests:**
```bash
cargo test --release
```

**Mint NFT with ZK proof:**
```bash
cargo run --release --bin mint-nft 12345
```
The secret (12345) generates a ZK proof. Only valid proofs can mint NFTs!

**Transfer NFT:**
```bash
cargo run --release --bin transfer-nft <MINT_ADDRESS> <RECIPIENT_ADDRESS>
```

## How It Works

### ZK Proof Circuit

The circuit proves knowledge of a secret without revealing it:
```rust
// Prover knows: secret
// Public: commitment = hash(secret)
// Proof: I know a secret that hashes to this commitment
```

### NFT Minting Flow

1. **Generate Proof**: User provides secret → generates ZK proof
2. **Verify Proof**: System verifies proof is valid
3. **Mint NFT**: If proof valid → mint NFT on Solana
4. **Create Metadata**: Add metadata from IPFS
5. **Master Edition**: Make it a unique NFT (supply = 1)

## Project Structure
```
token/
├── src/
│   ├── lib.rs              # NFT minter logic
│   ├── zk/
│   │   ├── mod.rs          # ZK proof system
│   │   └── circuit.rs      # ZK circuit definition
│   └── bin/
│       ├── setup.rs        # Generate ZK keys
│       ├── mint_nft.rs     # Mint with proof
│       └── transfer_nft.rs # Transfer NFT
├── tests/
│   └── integration_test.rs # Full test suite
└── Cargo.toml
```

## IPFS Metadata

NFT uses: `https://ipfs.io/ipfs/QmPhPb2Cp59ZfbmP2ZuoANghBTxF7KiBFJLZMZAES4evC2`

## View Your NFT

After minting, view on:
- Solana Explorer (devnet)
- Phantom Wallet (switch to devnet)
- Solflare Wallet (switch to devnet)

## Testing
```bash
# Run all tests
cargo test --release

# Run specific test
cargo test --release test_proof_generation_and_verification
```

## Security Notes

- This uses a simplified hash in the ZK circuit for demo purposes
- Production should use Poseidon or similar cryptographic hash
- Keep your wallet keypair secure
- The proving key should be generated via trusted setup in production

## License

MIT
