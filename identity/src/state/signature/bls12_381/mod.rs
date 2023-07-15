use ark_ec::CurveGroup;
use ark_ec::AffineRepr;
use ark_ff::{UniformRand, ToConstraintField, Field};
// use ark_ff::{Field, ToConstraintField, UniformRand};
use ark_serialize::CanonicalSerialize;
use ark_std::ops::Mul;
use ark_std::{end_timer, marker::PhantomData, start_timer};
use derivative::Derivative;
use digest::Digest;

use super::SignatureScheme;

// use super::hash::hash_to_curve;

pub type Error = Box<dyn ark_std::error::Error>;

pub struct Bls12381<C: CurveGroup, D: Digest> {
    _group: PhantomData<C>,
    _hash: PhantomData<D>,
}

#[derive(Derivative)]
#[derivative(Clone(bound = "C: CurveGroup, H: Digest"), Debug)]
pub struct Parameters<C: CurveGroup, H: Digest> {
    _hash: PhantomData<H>,
    pub generator: C::Affine,
    pub salt: [u8; 32],
}

pub type PublicKey<C> = <C as CurveGroup>::Affine;

#[derive(Clone, Default, Debug, CanonicalSerialize)]
pub struct SecretKey<C: CurveGroup>(pub C::ScalarField);

#[derive(Clone, Default, Debug)]
pub struct Signature<C: CurveGroup> {
    pub prover_response: C::ScalarField,
    pub verifier_challenge: C::ScalarField,
}

impl<C: CurveGroup, D: Digest> SignatureScheme for Bls12381<C, D>
{
    type Parameters = Parameters<C, D>;
    type PublicKey = PublicKey<C>;
    type SecretKey = SecretKey<C>;
    type Signature = Signature<C>;

    // https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L49

    fn setup<R: rand::Rng>(rng: &mut R) -> Result<Self::Parameters, Error> {
        let setup_time = start_timer!(|| "Signature::Setup");

        let mut salt = [0u8; 32];
        rng.fill_bytes(&mut salt);
        let generator = C::rand(rng).into();

        end_timer!(setup_time);

        Ok(Parameters {
            _hash: PhantomData,
            generator,
            salt,
        })
    }

    // https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L64
    fn keygen<R: rand::Rng>(
        parameters: &Self::Parameters,
        rng: &mut R,
    ) -> Result<(Self::PublicKey, Self::SecretKey), Error> {
        let keygen_time = start_timer!(|| "Signature::KeyGen");

        let secret_key = C::ScalarField::rand(rng);
        let public_key = parameters.generator.mul(secret_key).into();

        end_timer!(keygen_time);
        Ok((public_key, SecretKey(secret_key)))
    }

    // https://github.com/dusk-network/bls12_381-sign/blob/main/rust/bls12_381-sign/src/keys/secret.rs#L71
    // https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L92
    // https://github.com/kobigurk/zkhack-bls-pedersen/blob/main/src/hash.rs

    fn sign<R: rand::Rng>(
        _parameters: &Self::Parameters,
        _sk: &Self::SecretKey,
        _message: &[u8],
        _rng: &mut R,
    ) -> Result<Self::Signature, Error> {
        todo!()
        // ) -> Result<Self::Signature, ark_crypto_primitives::Error> {
        // let sign_time = start_timer!(|| "Signature::Sign");
        // let random_scalar: C::ScalarField = C::ScalarField::rand(rng);
        // let (_, h) = hash_to_curve(message);
        // let e = hashed * sk;
        // C::ScalarField::from_rand_bytes(&D::digest(&hash_input));
        // Ok(())
    }

    fn verify(
        _pp: &Self::Parameters,
        _pk: &Self::PublicKey,
        _message: &[u8],
        _signature: &Self::Signature,
    ) -> Result<bool, ark_crypto_primitives::Error> {
        todo!()
    }
}

pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    for byte in bytes {
        for i in 0..8 {
            let bit = (*byte >> (8 - i - 1)) & 1;
            bits.push(bit == 1);
        }
    }
    bits
}

impl<ConstraintF: Field, C: CurveGroup + ToConstraintField<ConstraintF>, D: Digest>
    ToConstraintField<ConstraintF> for Parameters<C, D>
{
    #[inline]
    fn to_field_elements(&self) -> Option<Vec<ConstraintF>> {
        self.generator.into_group().to_field_elements()
    }
}
