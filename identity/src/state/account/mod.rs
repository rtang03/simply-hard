pub mod constraints;

use ark_crypto_primitives::signature::schnorr::{PublicKey, SecretKey};
use ark_ed_on_bls12_381::EdwardsProjective as JubJub;
use ark_ff::{biginteger::BigInteger64 as B, BigInteger as _};
use ark_serialize::CanonicalSerialize;

/// Account public key used to verify transaction signatures.
pub type AccountPublicKey = PublicKey<JubJub>;

/// Account secret key used to create transaction signatures.
pub type AccountSecretKey = SecretKey<JubJub>;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug)]
pub struct AccountId(pub u64);

impl AccountId {
    ///
    /// see https://docs.rs/ark-ff/0.4.2/ark_ff/biginteger/trait.BigInteger.html#tymethod.to_bytes_le
    ///
    pub fn to_bytes_le(&self) -> Vec<u8> {
        B::from(self.0).to_bytes_le()
    }
    /// Increment the identifier in place.
    pub fn checked_increment(&mut self) -> Option<()> {
        self.0.checked_add(1).map(|result| self.0 = result)
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, Debug, CanonicalSerialize)]
pub struct Nonce(pub u64);

impl Nonce {
    pub fn to_bytes_le(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    pub fn checked_increment(&mut self) -> Option<()> {
        self.0.checked_add(1).map(|result| self.0 = result)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, CanonicalSerialize)]
pub struct AccountInformation {
    pub public_key: AccountPublicKey,
    pub nonce: Nonce,
}

impl AccountInformation {
    /// Convert the account information to bytes.
    pub fn to_bytes_le(&self) -> Vec<u8> {
        let mut uncompressed_bytes = Vec::new();
        self.serialize_uncompressed(&mut uncompressed_bytes)
            .unwrap();
        uncompressed_bytes
    }
}
