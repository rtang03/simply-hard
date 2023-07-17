use super::{hash::hash_to_curve, SignatureScheme};
use ark_bls12_381::{Bls12_381, Fq2, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, short_weierstrass::Affine, AffineRepr, CurveGroup};
use ark_ff::{BigInteger256, BigInteger384};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate,
};
use ark_std::{end_timer, io::Write, start_timer, UniformRand};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

// https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L49
// https://github.com/dusk-network/bls12_381-sign/blob/main/rust/bls12_381-sign/src/keys/secret.rs#L71
// https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L92
// https://github.com/kobigurk/zkhack-bls-pedersen/blob/main/src/hash.rs
// https://github.com/arkworks-rs/crypto-primitives/blob/main/src/signature/schnorr/mod.rs#L64

pub type Error = Box<dyn ark_std::error::Error>;

pub trait Parser {
    fn get_string(&self) -> String;
}

pub struct Bls12381 {}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub generator: Affine<ark_bls12_381::g2::Config>,
    pub seed: [u8; 32],
}

#[derive(Debug, Default, Hash, Clone, Eq, PartialEq)]
pub struct PublicKey(pub G2Affine);

impl Parser for PublicKey {
    fn get_string(&self) -> String {
        let mut pk_bytes = Vec::new();
        match self.serialize_uncompressed(&mut pk_bytes) {
            Ok(_) => hex::encode(pk_bytes),
            Err(_) => String::from("0"),
        }
    }
}

impl CanonicalSerialize for PublicKey {
    #[allow(unused_qualifications)]
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.x().unwrap().serialize_with_mode(writer, compress)
    }

    #[inline]
    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for PublicKey {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let x = Fq2::deserialize_with_mode(&mut reader, compress, validate)?;
        let a = G2Affine::get_point_from_x_unchecked(x, true).unwrap();

        // let a = G2Affine::deserialize_with_mode(&mut reader, compress, validate)?;
        Ok(Self(a))
    }
}

impl Valid for PublicKey {
    fn check(&self) -> Result<(), SerializationError> {
        self.0.check()?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SecretKey(pub BigInteger256);

impl Parser for SecretKey {
    fn get_string(&self) -> String {
        let mut sk_bytes = Vec::new();
        match self.serialize_uncompressed(&mut sk_bytes) {
            Ok(_) => hex::encode(sk_bytes),
            Err(_) => String::from("0"),
        }
    }
}

impl CanonicalSerialize for SecretKey {
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

impl CanonicalDeserialize for SecretKey {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let a = BigInteger256::deserialize_with_mode(&mut reader, compress, validate)?;
        Ok(Self(a))
    }
}

impl Valid for SecretKey {
    fn check(&self) -> Result<(), SerializationError> {
        self.0.check()?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Signature(pub G1Affine);

impl Parser for Signature {
    fn get_string(&self) -> String {
        let mut sig_bytes = Vec::new();
        match self.serialize_uncompressed(&mut sig_bytes) {
            Ok(_) => hex::encode(sig_bytes),
            Err(_) => String::from("0"),
        }
    }
}

impl CanonicalSerialize for Signature {
    #[allow(unused_qualifications)]
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.x().unwrap().serialize_with_mode(writer, compress)
    }

    #[inline]
    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl CanonicalDeserialize for Signature {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let x = BigInteger384::deserialize_with_mode(&mut reader, compress, validate)?;
        let a = G1Affine::get_point_from_x_unchecked(x.into(), true).unwrap();
        Ok(Self(a))
    }
}

impl Valid for Signature {
    fn check(&self) -> Result<(), SerializationError> {
        self.0.check()?;
        Ok(())
    }
}

impl SignatureScheme for Bls12381 {
    type Parameters = Parameters;
    type PublicKey = PublicKey;
    type SecretKey = SecretKey;
    type Signature = Signature;

    fn load_secret_key(secret_key: String) -> Self::SecretKey {
        if secret_key.len() != 64 {
            return SecretKey(BigInteger256::zero());
        }

        let decoded = match hex::decode(secret_key) {
            Ok(val) => val,
            Err(_) => vec![0u8; 32],
        };

        match SecretKey::deserialize_uncompressed(&*decoded) {
            Ok(val) => val,
            Err(_) => SecretKey(BigInteger256::zero()),
        }
    }

    fn load_public_key(public_key: String) -> Self::PublicKey {
        if public_key.len() != 192 {
            return PublicKey(G2Affine::zero());
        }

        let decoded = match hex::decode(public_key) {
            Ok(val) => val,
            Err(_) => vec![0u8; 96],
        };

        match PublicKey::deserialize_uncompressed(&*decoded) {
            Ok(val) => val,
            Err(_) => PublicKey(G2Affine::zero()),
        }
    }

    fn load_signature(signature: String) -> Self::Signature {
        if signature.len() != 96 {
            return Signature(G1Affine::zero());
        }

        let decoded = match hex::decode(signature) {
            Ok(val) => val,
            Err(_) => vec![0u8; 48],
        };

        match Signature::deserialize_uncompressed(&*decoded) {
            Ok(val) => val,
            Err(_) => Signature(G1Affine::zero()),
        }
    }

    fn setup(seed: [u8; 32]) -> Result<Self::Parameters, Error> {
        let setup_time = start_timer!(|| "Signature::Setup");
        let generator: Affine<ark_bls12_381::g2::Config> = G2Affine::generator();

        end_timer!(setup_time);

        Ok(Parameters { generator, seed })
    }

    fn keygen(parameters: &Self::Parameters) -> Result<(Self::PublicKey, Self::SecretKey), Error> {
        let keygen_time = start_timer!(|| "Signature::KeyGen");
        let rng = &mut ChaCha20Rng::from_seed(parameters.seed);
        let sk: BigInteger256 = Fr::rand(rng).into();

        let public_key = parameters.generator.mul_bigint(sk).into();

        end_timer!(keygen_time);
        Ok((PublicKey(public_key), SecretKey(sk)))
    }

    fn sign(
        parameters: &Self::Parameters,
        sk: &Self::SecretKey,
        message: &[u8],
    ) -> Result<Self::Signature, Error> {
        let sign_time = start_timer!(|| "Signature::Sign");
        let (_, h) = hash_to_curve(parameters.seed, message);
        let e = G1Affine::mul_bigint(&h, sk.0).into_affine();

        end_timer!(sign_time);
        Ok(Signature(e))
    }

    fn verify(
        parameters: &Self::Parameters,
        pk: &Self::PublicKey,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<bool, Error> {
        let verify_time = start_timer!(|| "Signature::Verify");
        let a = Bls12_381::pairing(signature.0, G2Affine::generator());
        let (_, h) = hash_to_curve(parameters.seed, message);
        let b = Bls12_381::pairing(h, pk.0);
        end_timer!(verify_time);
        Ok(a == b)
    }
}
