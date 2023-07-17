pub mod bls12_381;
pub mod hash;

// https://github.com/arkworks-rs/std/blob/master/src/rand_helper.rs
// https://github.com/arkworks-rs/algebra/blob/master/ff/README.md
// https://github.com/kobigurk/zkhack-bls-pedersen

use crate::Error;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::Rng;

use self::bls12_381::{PublicKey, Signature};

pub trait SignatureScheme {
    type Parameters: Clone;
    // type PublicKey: CanonicalSerialize + Hash + Eq + Clone + Default + std::fmt::Debug;
    type SecretKey: CanonicalSerialize + Clone + std::fmt::Debug;
    // type Signature: Clone + std::fmt::Debug;

    fn setup<R: Rng>(rng: &mut R) -> Result<Self::Parameters, Error>;

    fn keygen<R: Rng>(
        parameters: &Self::Parameters,
        rng: &mut R,
    ) -> Result<(PublicKey, Self::SecretKey), Error>;

    fn sign<R: Rng>(
        parameters: &Self::Parameters,
        sk: &Self::SecretKey,
        message: &[u8],
        rng: &mut R,
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
    use crate::state::signature::{bls12_381, SignatureScheme};
    use ark_bls12_381::{G1Projective, G2Projective};
    use ark_serialize::CanonicalSerialize;
    use ark_std::test_rng;
    use blake2::Blake2s256 as Blake2s;

    fn sign_and_verify<S: SignatureScheme>(message: &[u8]) {
        let rng = &mut test_rng();
        let parameters = S::setup::<_>(rng).unwrap();
        let (pk, sk) = S::keygen(&parameters, rng).unwrap();
        let sig = S::sign(&parameters, &sk, message, rng).unwrap();
        let mut pk_bytes = Vec::new();
        pk.serialize_uncompressed(&mut pk_bytes).unwrap();
        println!("public key {:?}", pk_bytes);
        let mut sig_bytes = Vec::new();
        sig.serialize_uncompressed(&mut sig_bytes).unwrap();
        println!("signature {:?}", sig_bytes);
        assert!(S::verify(&parameters, &pk, message, &sig).unwrap());
    }

    #[test]
    fn test_keygen() {
        let message = "Hi, I am a Schnorr signature!";
        let _rng = &mut test_rng();
        sign_and_verify::<bls12_381::Bls12381<G1Projective, G2Projective, Blake2s>>(
            message.as_bytes(),
        );
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
