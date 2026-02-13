pub mod circuit;

use anyhow::{Context, Result};
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use std::fs;
use std::path::Path;

use circuit::{hash_native, MintAuthCircuit};

pub struct ZkProver {
    pub proving_key: ProvingKey<Bn254>,
}

pub struct ZkVerifier {
    pub prepared_vk: PreparedVerifyingKey<Bn254>,
}

impl ZkProver {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = fs::read(path)?;
        let proving_key = ProvingKey::deserialize_compressed(&bytes[..])?;
        Ok(Self { proving_key })
    }

    pub fn generate_proof(&self, secret: u64) -> Result<(Proof<Bn254>, Fr)> {
        let secret_field = Fr::from(secret);
        let commitment = hash_native(secret_field);

        let circuit = MintAuthCircuit {
            secret: Some(secret_field),
            commitment: Some(commitment),
        };

        let mut rng = thread_rng();
        let proof = Groth16::<Bn254>::prove(&self.proving_key, circuit, &mut rng)
            .context("Failed to generate proof")?;

        Ok((proof, commitment))
    }
}

impl ZkVerifier {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = fs::read(path)?;
        let vk = VerifyingKey::deserialize_compressed(&bytes[..])?;
        let prepared_vk = Groth16::<Bn254>::process_vk(&vk)?;
        Ok(Self { prepared_vk })
    }

    pub fn verify(&self, proof: &Proof<Bn254>, commitment: Fr) -> Result<bool> {
        let public_inputs = vec![commitment];
        let result =
            Groth16::<Bn254>::verify_with_processed_vk(&self.prepared_vk, &public_inputs, proof)?;
        Ok(result)
    }
}

pub fn setup_keys() -> Result<()> {
    println!("🔧 Setting up ZK parameters...");

    let circuit = MintAuthCircuit::<Fr> {
        secret: None,
        commitment: None,
    };

    let mut rng = thread_rng();
    let (pk, vk) =
        Groth16::<Bn254>::circuit_specific_setup(circuit, &mut rng).context("Setup failed")?;

    // Save proving key
    let mut pk_bytes = Vec::new();
    pk.serialize_compressed(&mut pk_bytes)?;
    fs::write("proving_key.bin", pk_bytes)?;
    println!("✅ Proving key saved to proving_key.bin");

    // Save verifying key
    let mut vk_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_bytes)?;
    fs::write("verifying_key.bin", vk_bytes)?;
    println!("✅ Verifying key saved to verifying_key.bin");

    Ok(())
}
