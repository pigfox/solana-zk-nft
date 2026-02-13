pub mod zk;

use anyhow::{anyhow, Result};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{Creator, DataV2},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};
use std::str::FromStr;

const METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

pub struct NftMinter {
    pub client: RpcClient,
    pub payer: Keypair,
}

impl NftMinter {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let client =
            RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

        let wallet_path = shellexpand::tilde("~/.config/solana/id.json");
        let payer = read_keypair_file(&*wallet_path)
            .map_err(|e| anyhow!("Failed to read keypair file: {}", e))?;

        Ok(Self { client, payer })
    }

    pub fn check_balance(&self) -> Result<f64> {
        let balance = self.client.get_balance(&self.payer.pubkey())?;
        Ok(balance as f64 / 1_000_000_000.0)
    }

    pub fn mint_nft(&self, uri: &str) -> Result<Pubkey> {
        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();

        println!("🪙 Creating mint: {}", mint_pubkey);

        // Derive PDAs
        let metadata_program = Pubkey::from_str(METADATA_PROGRAM_ID)?;
        let (metadata_pda, _) = Pubkey::find_program_address(
            &[b"metadata", metadata_program.as_ref(), mint_pubkey.as_ref()],
            &metadata_program,
        );

        let (master_edition_pda, _) = Pubkey::find_program_address(
            &[
                b"metadata",
                metadata_program.as_ref(),
                mint_pubkey.as_ref(),
                b"edition",
            ],
            &metadata_program,
        );

        let token_account = spl_associated_token_account::get_associated_token_address(
            &self.payer.pubkey(),
            &mint_pubkey,
        );

        // Step 1: Create mint
        let rent = self
            .client
            .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

        let create_mint_ix = system_instruction::create_account(
            &self.payer.pubkey(),
            &mint_pubkey,
            rent,
            spl_token::state::Mint::LEN as u64,
            &spl_token::id(),
        );

        let init_mint_ix = initialize_mint(
            &spl_token::id(),
            &mint_pubkey,
            &self.payer.pubkey(),
            Some(&self.payer.pubkey()),
            0,
        )?;

        let mut tx = Transaction::new_with_payer(
            &[create_mint_ix, init_mint_ix],
            Some(&self.payer.pubkey()),
        );

        let recent_blockhash = self.client.get_latest_blockhash()?;
        tx.sign(&[&self.payer, &mint], recent_blockhash);

        self.client.send_and_confirm_transaction(&tx)?;
        println!("✅ Mint created");

        // Step 2: Create ATA
        let create_ata_ix = create_associated_token_account(
            &self.payer.pubkey(),
            &self.payer.pubkey(),
            &mint_pubkey,
            &spl_token::id(),
        );

        let mut tx = Transaction::new_with_payer(&[create_ata_ix], Some(&self.payer.pubkey()));
        tx.sign(&[&self.payer], self.client.get_latest_blockhash()?);
        self.client.send_and_confirm_transaction(&tx)?;
        println!("✅ Token account created");

        // Step 3: Mint token
        let mint_to_ix = mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &token_account,
            &self.payer.pubkey(),
            &[],
            1,
        )?;

        let mut tx = Transaction::new_with_payer(&[mint_to_ix], Some(&self.payer.pubkey()));
        tx.sign(&[&self.payer], self.client.get_latest_blockhash()?);
        self.client.send_and_confirm_transaction(&tx)?;
        println!("✅ Token minted");

        // Step 4: Create metadata
        let metadata_data = DataV2 {
            name: "ZK Proof NFT".to_string(),
            symbol: "ZKNFT".to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 500,
            creators: Some(vec![Creator {
                address: self.payer.pubkey(),
                verified: true,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        let create_metadata_ix = CreateMetadataAccountV3 {
            metadata: metadata_pda,
            mint: mint_pubkey,
            mint_authority: self.payer.pubkey(),
            payer: self.payer.pubkey(),
            update_authority: (self.payer.pubkey(), true),
            system_program: solana_sdk::system_program::id(),
            rent: None,
        }
        .instruction(CreateMetadataAccountV3InstructionArgs {
            data: metadata_data,
            is_mutable: true,
            collection_details: None,
        });

        let mut tx = Transaction::new_with_payer(&[create_metadata_ix], Some(&self.payer.pubkey()));
        tx.sign(&[&self.payer], self.client.get_latest_blockhash()?);
        self.client.send_and_confirm_transaction(&tx)?;
        println!("✅ Metadata created");

        // Step 5: Create master edition
        let create_master_edition_ix = CreateMasterEditionV3 {
            edition: master_edition_pda,
            mint: mint_pubkey,
            update_authority: self.payer.pubkey(),
            mint_authority: self.payer.pubkey(),
            payer: self.payer.pubkey(),
            metadata: metadata_pda,
            token_program: spl_token::id(),
            system_program: solana_sdk::system_program::id(),
            rent: None,
        }
        .instruction(CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        });

        let mut tx =
            Transaction::new_with_payer(&[create_master_edition_ix], Some(&self.payer.pubkey()));
        tx.sign(&[&self.payer], self.client.get_latest_blockhash()?);
        self.client.send_and_confirm_transaction(&tx)?;
        println!("✅ Master edition created");

        Ok(mint_pubkey)
    }

    pub fn transfer_nft(&self, mint: &Pubkey, recipient: &Pubkey) -> Result<()> {
        let from_ata =
            spl_associated_token_account::get_associated_token_address(&self.payer.pubkey(), mint);

        let to_ata = spl_associated_token_account::get_associated_token_address(recipient, mint);

        // Create recipient ATA if needed
        let create_ata_ix = create_associated_token_account(
            &self.payer.pubkey(),
            recipient,
            mint,
            &spl_token::id(),
        );

        // Transfer instruction
        let transfer_ix = spl_token::instruction::transfer(
            &spl_token::id(),
            &from_ata,
            &to_ata,
            &self.payer.pubkey(),
            &[],
            1,
        )?;

        let mut tx =
            Transaction::new_with_payer(&[create_ata_ix, transfer_ix], Some(&self.payer.pubkey()));

        tx.sign(&[&self.payer], self.client.get_latest_blockhash()?);
        self.client.send_and_confirm_transaction(&tx)?;

        Ok(())
    }
}
