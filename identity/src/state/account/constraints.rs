// NOTE:
// see https://githb.com/arkworks-rs/r1cs-tutorial/blob/main/rollup/src/account.rs

use super::{AccountId, Nonce};
use crate::state::ConstraintF;
use ark_crypto_primitives::signature::schnorr::constraints::PublicKeyVar;
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};
use ark_r1cs_std::bits::uint64::UInt64;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSystem, Namespace, SynthesisError};
use std::borrow::Borrow;

/// Account public key used to verify transaction signatures.
pub type AccountPublicKeyVar = PublicKeyVar<EdwardsProjective, EdwardsVar>;

/// Account identifier
#[derive(Clone, Debug)]
pub struct AccountIdVar(pub UInt64<ConstraintF>);

impl AccountIdVar {
    /// Convert the account identifier to bytes.
    #[tracing::instrument(skip(self))]
    pub fn to_bytes_le(&self) -> Vec<UInt8<ConstraintF>> {
        self.0.to_bytes().unwrap()
    }
}

impl AllocVar<AccountId, ConstraintF> for AccountIdVar {
    #[tracing::instrument(skip(cs, f, mode))]
    fn new_variable<T: Borrow<AccountId>>(
        cs: impl Into<Namespace<ConstraintF>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        UInt64::new_variable(cs, || f().map(|u| u.borrow().0), mode).map(Self)
    }
}

/// Information about the account, such as the balance and the associated public key.
#[derive(Clone)]
pub struct AccountInformationVar {
    /// The account public key.
    pub public_key: AccountPublicKeyVar,
    // The nonce associated with this this account.
    pub nonce: NonceVar,
}

/// Represents transaction amounts and account balances.
#[derive(Clone, Debug)]
pub struct NonceVar(pub UInt64<ConstraintF>);

impl NonceVar {
    #[tracing::instrument(skip(self))]
    pub fn to_bytes_le(&self) -> Vec<UInt8<ConstraintF>> {
        self.0.to_bytes().unwrap()
    }

    #[tracing::instrument(skip(self))]
    pub fn checked_increment(&mut self) -> Result<Self, SynthesisError> {
        // To do a checked add, we add two uint64's directly.
        // We also check for overflow, by casting them to field elements,
        // adding the field element representation
        // converting the field elements to bits
        // and then checking if the 65th bit is 0.
        // TODO: Demonstrate via circuit profiling if this needs optimization.
        let cs = ConstraintSystem::<ConstraintF>::new_ref();
        let one = UInt64::new_constant(cs, 1).unwrap();
        let self_bits = self.0.to_bits_le();
        let self_fe = Boolean::le_bits_to_fp_var(&self_bits)?;
        let one_bits = one.to_bits_le();
        let one_fe = Boolean::le_bits_to_fp_var(&one_bits)?;
        let res_fe = self_fe + one_fe;
        let res_bz = res_fe.to_bytes()?;
        // Ensure 65th bit is 0
        // implies 8th word (0-indexed) is 0
        res_bz[8].enforce_equal(&UInt8::<ConstraintF>::constant(0))?;
        // Add sum
        let result = UInt64::addmany(&[self.0.clone(), one])?;
        Ok(NonceVar(result))
    }
}

impl AllocVar<Nonce, ConstraintF> for NonceVar {
    #[tracing::instrument(skip(cs, f, mode))]
    fn new_variable<T: Borrow<Nonce>>(
        cs: impl Into<Namespace<ConstraintF>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        UInt64::new_variable(cs.into(), || f().map(|u| u.borrow().0), mode).map(Self)
    }
}
