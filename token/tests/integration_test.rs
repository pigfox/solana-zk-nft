use ark_bn254::Fr;
use solana_zk_nft::zk::{setup_keys, ZkProver, ZkVerifier};
use std::path::PathBuf;

fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn ensure_keys_exist() {
    let root = get_project_root();
    let pk_path = root.join("proving_key.bin");
    let vk_path = root.join("verifying_key.bin");

    if !pk_path.exists() || !vk_path.exists() {
        let _guard = std::env::set_current_dir(&root);
        setup_keys().unwrap();
    }
}

#[test]
fn test_zk_setup() {
    let root = get_project_root();
    let _guard = std::env::set_current_dir(&root);

    let _ = std::fs::remove_file("proving_key.bin");
    let _ = std::fs::remove_file("verifying_key.bin");

    setup_keys().unwrap();

    assert!(std::path::Path::new("proving_key.bin").exists());
    assert!(std::path::Path::new("verifying_key.bin").exists());
}

#[test]
fn test_proof_generation_and_verification() {
    ensure_keys_exist();

    let root = get_project_root();
    let pk_path = root.join("proving_key.bin");
    let vk_path = root.join("verifying_key.bin");

    let secret = 42u64;

    let prover = ZkProver::from_file(&pk_path).unwrap();
    let verifier = ZkVerifier::from_file(&vk_path).unwrap();

    let (proof, commitment) = prover.generate_proof(secret).unwrap();

    let is_valid = verifier.verify(&proof, commitment).unwrap();
    assert!(is_valid);
}

#[test]
fn test_invalid_proof_fails() {
    ensure_keys_exist();

    let root = get_project_root();
    let pk_path = root.join("proving_key.bin");
    let vk_path = root.join("verifying_key.bin");

    let secret = 42u64;

    let prover = ZkProver::from_file(&pk_path).unwrap();
    let verifier = ZkVerifier::from_file(&vk_path).unwrap();

    let (proof, _) = prover.generate_proof(secret).unwrap();

    // Use wrong commitment
    let wrong_commitment = Fr::from(99999u64);
    let is_valid = verifier.verify(&proof, wrong_commitment).unwrap();

    assert!(!is_valid);
}
