pub mod constraints;

use super::{
    account::{AccountId, AccountPublicKey, AccountSecretKey, Nonce},
    ledger::{Parameters, SignatureParameters, State},
};
use ark_crypto_primitives::signature::{
    schnorr::{Schnorr, Signature},
    SignatureScheme,
};
use ark_ed_on_bls12_381::EdwardsProjective as JubJub;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::Rng;
use blake2::Blake2s256 as Blake2s;

/// Transaction transferring some amount from one account to another.
#[derive(Clone, Debug)]
pub struct Transaction {
    /// The account information of the sender.
    pub sender: AccountId,
    /// The next available nonce
    pub nonce: Nonce,
    /// The spend authorization is a signature over the sender, the recipient,
    /// and the amount.
    pub signature: Signature<JubJub>,
}

impl Transaction {
    /// Verify just the signature in the transaction.
    fn verify_signature(&self, pp: &SignatureParameters, pub_key: &AccountPublicKey) -> bool {
        // The authorized message consists of
        // (SenderAccId || SenderPubKey || Nonce )
        let mut message = self.sender.to_bytes_le();
        message.extend(self.nonce.to_bytes_le());
        <Schnorr<JubJub, Blake2s> as SignatureScheme>::verify(
            pp,
            pub_key,
            &message,
            &self.signature,
        )
        .unwrap()
    }

    /// Check that the transaction is valid for the given ledger state. Condition
    /// 1. Verify that the signature is valid with respect to the public key
    /// corresponding to `self.sender`.
    /// 2. check if the sender's request nonce is same as corresponding state leaf
    pub fn validate(&self, parameters: &Parameters, state: &State) -> bool {
        // Lookup public key corresponding to sender ID
        if let Some(sender_acc_info) = state.id_to_account_info.get(&self.sender) {
            let mut result = true;
            // Check that the account_info exists in the Merkle tree.
            result &= {
                let path = state
                    .account_merkle_tree
                    .generate_proof(self.sender.0 as usize)
                    .expect("path should exist");
                let mut uncompressed_bytes = Vec::new();
                sender_acc_info
                    .serialize_uncompressed(&mut uncompressed_bytes)
                    .unwrap();
                path.verify(
                    &parameters.leaf_crh_params,
                    &parameters.two_to_one_crh_params,
                    &state.account_merkle_tree.root(),
                    uncompressed_bytes.as_slice(),
                )
                .unwrap()
            };
            // Verify the nonce is valid
            result &= sender_acc_info.nonce.0 == self.nonce.0;
            // Verify the signature against the sender pubkey.
            result &= self.verify_signature(&parameters.sig_params, &sender_acc_info.public_key);
            result
        } else {
            false
        }
    }

    // Create a (possibly invalid) transaction.
    pub fn new<R: Rng>(
        parameters: &Parameters,
        sender: AccountId,
        nonce: Nonce,
        sender_sk: &AccountSecretKey,
        rng: &mut R,
    ) -> Self {
        let mut message = sender.to_bytes_le();
        message.extend(nonce.to_bytes_le());
        let signature = <Schnorr<JubJub, Blake2s> as SignatureScheme>::sign(
            &parameters.sig_params,
            sender_sk,
            &message,
            rng,
        )
        .unwrap();
        Self {
            sender,
            nonce,
            signature,
        }
    }
}
