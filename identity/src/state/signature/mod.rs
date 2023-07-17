pub mod bls12_381;
pub mod hash;

pub use self::bls12_381::{PublicKey, SecretKey, Signature};
use crate::Error;

// NOTE: this implementation is not compliant with irtf spec
// https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-bls-signature-05
// https://github.com/arkworks-rs/std/blob/master/src/rand_helper.rs
// https://github.com/arkworks-rs/algebra/blob/master/ff/README.md
// https://github.com/kobigurk/zkhack-bls-pedersen

// FIXME: need to handle rogue-key attack

pub trait SignatureScheme {
    type Parameters: Clone;

    fn setup(seed: [u8; 32]) -> Result<Self::Parameters, Error>;

    fn keygen(parameters: &Self::Parameters) -> Result<(PublicKey, SecretKey), Error>;

    fn sign(
        parameters: &Self::Parameters,
        sk: &SecretKey,
        message: &[u8],
    ) -> Result<Signature, Error>;

    fn verify(
        parameters: &Self::Parameters,
        pk: &PublicKey,
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, Error>;
}

#[cfg(test)]
mod test {
    pub use self::bls12_381::{PublicKey, SecretKey, Signature};
    use crate::state::signature::{bls12_381, SignatureScheme};

    fn sign_and_verify<S: SignatureScheme>(message: &[u8]) {
        let parameters = S::setup([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ])
        .unwrap();
        let (pk, sk) = S::keygen(&parameters).unwrap();

        // reload secret_key from String
        let loaded_secretkey = SecretKey::new(sk.get_string());

        let sig = S::sign(&parameters, &loaded_secretkey, message).unwrap();

        println!("public key {:?}", pk.get_string());
        println!("secret key {:?}", sk.get_string());
        println!("signature {:?}", sig.get_string());

        // reload signature from String
        let loaded_signature = Signature::new(sig.get_string());
        let loaded_publickey = PublicKey::new(pk.get_string());

        assert!(S::verify(&parameters, &loaded_publickey, message, &loaded_signature).unwrap());
    }

    fn failed_verification<S: SignatureScheme>(message: &[u8], bad_message: &[u8]) {
        let parameters = S::setup([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ])
        .unwrap();
        let (pk, sk) = S::keygen(&parameters).unwrap();
        let sig = S::sign(&parameters, &sk, message).unwrap();
        assert!(!S::verify(&parameters, &pk, bad_message, &sig).unwrap());
    }

    #[test]
    fn test_bls12381_signature() {
        let message = "Hi, I am a Schnorr signature!";
        sign_and_verify::<bls12_381::Bls12381>(message.as_bytes());
        failed_verification::<bls12_381::Bls12381>(message.as_bytes(), "Bad message".as_bytes());
    }
}

// FIXME: below code may be useful for Ed_on_bls12_381
// But no use here while using BLS12381
// #[test]
// fn test_curve() -> Result<(), ark_relations::r1cs::SynthesisError> {
//     use ark_ed_on_bls12_381::{constraints::*, *};
//     use ark_r1cs_std::prelude::*;
//     use ark_relations::r1cs::*;
//     use ark_std::UniformRand;

//     let cs = ConstraintSystem::<Fq>::new_ref();
//     let mut rng = ark_std::test_rng();
//     let a_native = Fq::rand(&mut rng);
//     let b_native = Fq::rand(&mut rng);
//     let a = FqVar::new_witness(ark_relations::ns!(cs, "generate_a"), || Ok(a_native))?;
//     let b = FqVar::new_witness(ark_relations::ns!(cs, "generate_b"), || Ok(b_native))?;
//     let a_const = FqVar::new_constant(ark_relations::ns!(cs, "a_as_constant"), a_native)?;
//     let b_const = FqVar::new_constant(ark_relations::ns!(cs, "b_as_constant"), b_native)?;
//     let one = FqVar::one();
//     let zero = FqVar::zero();
//     let two = &one + &one + &zero;
//     two.enforce_equal(&one.double()?)?;
//     assert!(cs.is_satisfied()?);
//     assert_eq!((&a + &b).value()?, a_native + b_native);
//     assert_eq!((&a * &b).value()?, a_native * b_native);
//     (&a + &b).enforce_equal(&(&a_const + &b_const))?;
//     assert!(cs.is_satisfied()?);
//     Ok(())
// }

// #[test]
// fn test_curve_2() -> Result<(), ark_relations::r1cs::SynthesisError> {
//     use ark_ed_on_bls12_381::{constraints::*, *};
//     use ark_r1cs_std::prelude::*;
//     use ark_relations::r1cs::*;
//     use ark_std::UniformRand;

//     let cs = ConstraintSystem::<Fq>::new_ref();
//     let mut rng = ark_std::test_rng();
//     let a_native = EdwardsProjective::rand(&mut rng);
//     let b_native = EdwardsProjective::rand(&mut rng);
//     let a = EdwardsVar::new_witness(ark_relations::ns!(cs, "generate_a"), || Ok(a_native))?;
//     let b = EdwardsVar::new_witness(ark_relations::ns!(cs, "generate_b"), || Ok(b_native))?;
//     let a_const = EdwardsVar::new_constant(ark_relations::ns!(cs, "a_as_constant"), a_native)?;
//     let b_const = EdwardsVar::new_constant(ark_relations::ns!(cs, "b_as_constant"), b_native)?;
//     let zero = EdwardsVar::zero();
//     let two_a = &a + &a + &zero;
//     two_a.enforce_equal(&a.double()?)?;
//     assert!(cs.is_satisfied()?);
//     assert_eq!((&a + &b).value()?, a_native + b_native);
//     (&a + &b).enforce_equal(&(&a_const + &b_const))?;
//     assert!(cs.is_satisfied()?);
//     Ok(())
// }
