use super::hash::hash_to_curve;
use super::SignatureScheme;
use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::short_weierstrass::Affine;
use ark_ec::AffineRepr;
use ark_ec::{pairing::Pairing, CurveGroup};
use ark_ff::BigInteger256;
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Write,
};
use ark_std::{end_timer, marker::PhantomData, start_timer};
use derivative::Derivative;
use digest::Digest;

pub type Error = Box<dyn ark_std::error::Error>;

pub struct Bls12381<G1: CurveGroup, G2: CurveGroup, D: Digest> {
    _group1: PhantomData<G1>,
    _group2: PhantomData<G2>,
    _hash: PhantomData<D>,
}

#[derive(Derivative)]
#[derivative(Clone(bound = "G2: CurveGroup, H: Digest"), Debug)]
pub struct Parameters<G2: CurveGroup, H: Digest> {
    _hash: PhantomData<H>,
    _g2: PhantomData<G2>,
    pub generator: Affine<ark_bls12_381::g2::Config>,
}

#[derive(Debug, Default, Hash, Clone, Eq, PartialEq)]
pub struct PublicKey(pub G2Affine);

impl CanonicalSerialize for PublicKey {
    #[allow(unused_qualifications)]
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    #[inline]
    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

#[derive(Clone, Debug, CanonicalSerialize)]
pub struct SecretKey(pub BigInteger256);

#[derive(Clone, Debug, CanonicalDeserialize)]
pub struct Signature(pub G1Affine);

impl CanonicalSerialize for Signature {
    #[allow(unused_qualifications)]
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    #[inline]
    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl<G1: CurveGroup, G2: CurveGroup, D: Digest> SignatureScheme for Bls12381<G1, G2, D> {
    type Parameters = Parameters<G2, D>;
    type SecretKey = SecretKey;
    // type Signature = G1Affine;

    // https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L49

    fn setup<R: rand::Rng>(_rng: &mut R) -> Result<Self::Parameters, Error> {
        let setup_time = start_timer!(|| "Signature::Setup");
        let generator: Affine<ark_bls12_381::g2::Config> = G2Affine::generator();
        // let generator = G2::rand(rng).into();

        end_timer!(setup_time);

        Ok(Parameters {
            _hash: PhantomData,
            _g2: PhantomData,
            generator,
        })
    }

    // https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L64
    fn keygen<R: rand::Rng>(
        parameters: &Self::Parameters,
        _rng: &mut R,
    ) -> Result<(PublicKey, Self::SecretKey), Error> {
        let keygen_time = start_timer!(|| "Signature::KeyGen");

        // let s: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FqConfig, 6>, 6> = Fq::rand(rng);
        // let secret_key = G2::ScalarField::rand(rng);

        let sk = BigInteger256::new([
            10959161122836963499,
            16025397983774428988,
            11799319322118777553,
            451810304166108882,
        ]);

        let public_key = parameters.generator.mul_bigint(sk).into();

        end_timer!(keygen_time);
        Ok((PublicKey(public_key), SecretKey(sk)))
    }

    // https://github.com/dusk-network/bls12_381-sign/blob/main/rust/bls12_381-sign/src/keys/secret.rs#L71
    // https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L92
    // https://github.com/kobigurk/zkhack-bls-pedersen/blob/main/src/hash.rs

    fn sign<R: rand::Rng>(
        _parameters: &Self::Parameters,
        sk: &Self::SecretKey,
        message: &[u8],
        _rng: &mut R,
    ) -> Result<Signature, Error> {
        let sign_time = start_timer!(|| "Signature::Sign");
        let (_, h) = hash_to_curve(message);
        let e = G1Affine::mul_bigint(&h, sk.0).into_affine();

        end_timer!(sign_time);
        Ok(Signature(e))
    }

    fn verify(
        _pp: &Self::Parameters,
        pk: &PublicKey,
        message: &[u8],
        signature: &Signature,
    ) -> Result<bool, Error> {
        let verify_time = start_timer!(|| "Signature::Verify");
        let a = Bls12_381::pairing(signature.0, G2Affine::generator());
        let (_, h) = hash_to_curve(message);
        let b = Bls12_381::pairing(h, pk.0);
        end_timer!(verify_time);
        Ok(a == b)
    }
}

