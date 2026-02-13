use ark_ff::PrimeField;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

/// Circuit that proves knowledge of a secret that hashes to a public commitment
#[derive(Clone)]
pub struct MintAuthCircuit<F: PrimeField> {
    /// Private: the secret
    pub secret: Option<F>,

    /// Public: commitment = hash(secret)
    pub commitment: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for MintAuthCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate private input
        let secret_var = FpVar::new_witness(cs.clone(), || {
            self.secret.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate public input
        let commitment_var = FpVar::new_input(cs.clone(), || {
            self.commitment.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Compute hash of secret (simplified hash for demo)
        let computed_hash = hash_field_element(&secret_var)?;

        // Verify: commitment == hash(secret)
        computed_hash.enforce_equal(&commitment_var)?;

        Ok(())
    }
}

/// Simplified hash function (in production, use Poseidon or similar)
fn hash_field_element<F: PrimeField>(input: &FpVar<F>) -> Result<FpVar<F>, SynthesisError> {
    // This is a placeholder - squares the input as a simple "hash"
    // In production, use proper hash like Poseidon
    let squared = input * input;
    let cubed = &squared * input;
    Ok(cubed)
}

/// Native hash function (must match circuit hash)
pub fn hash_native<F: PrimeField>(secret: F) -> F {
    // Must match the circuit hash
    let squared = secret * secret;
    squared * secret
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn test_circuit_satisfiability() {
        let secret = Fr::from(12345u64);
        let commitment = hash_native(secret);

        let circuit = MintAuthCircuit {
            secret: Some(secret),
            commitment: Some(commitment),
        };

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_fails_with_wrong_commitment() {
        let secret = Fr::from(12345u64);
        let wrong_commitment = Fr::from(99999u64);

        let circuit = MintAuthCircuit {
            secret: Some(secret),
            commitment: Some(wrong_commitment),
        };

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        assert!(!cs.is_satisfied().unwrap());
    }
}
