use super::{
    account::{AccountId, AccountInformation, AccountPublicKey, AccountSecretKey, Nonce},
    transaction::Transaction,
};
use crate::merkle_tree::{CompressH, JubJubMerkleTree, LeafH, SimplePath};
use ark_crypto_primitives::{
    crh::{CRHScheme, TwoToOneCRHScheme},
    merkle_tree::MerkleTree,
    signature::{
        schnorr::{self, PublicKey, Schnorr},
        SignatureScheme,
    },
};
use ark_ed_on_bls12_381::EdwardsProjective as JubJub;
use ark_serialize::CanonicalSerialize;
use ark_std::{log2, rand::Rng};
use blake2::Blake2s256 as Blake2s;
use std::collections::HashMap;

pub type SignatureParameters = schnorr::Parameters<JubJub, Blake2s>;

/// The parameters that are used in transaction creation and validation.
#[derive(Clone)]
pub struct Parameters {
    pub sig_params: SignatureParameters,
    pub leaf_crh_params: <LeafH as CRHScheme>::Parameters,
    pub two_to_one_crh_params: <CompressH as TwoToOneCRHScheme>::Parameters,
}

impl Parameters {
    pub fn sample<R: Rng>(rng: &mut R) -> Self {
        let sig_params = <Schnorr<JubJub, Blake2s> as SignatureScheme>::setup(rng).unwrap();
        let leaf_crh_params = <LeafH as CRHScheme>::setup(rng).unwrap();
        let two_to_one_crh_params = <CompressH as TwoToOneCRHScheme>::setup(rng).unwrap();
        Self {
            sig_params,
            leaf_crh_params,
            two_to_one_crh_params,
        }
    }
}

/// A Merkle tree containing account information.
pub type AccMerkleTree = JubJubMerkleTree;
pub type AccRoot = <CompressH as TwoToOneCRHScheme>::Output;
pub type AccPath = SimplePath;

#[derive(Clone)]
pub struct State {
    /// What is the next available account identifier?
    pub next_available_account: Option<AccountId>,
    /// A merkle tree mapping where the i-th leaf corresponds to the i-th account's
    /// information (= balance and public key).
    pub account_merkle_tree: AccMerkleTree,
    /// A mapping from an account's identifier to its information (= balance and public key).
    pub id_to_account_info: HashMap<AccountId, AccountInformation>,
    /// A mapping from a public key to an account's identifier.
    pub pub_key_to_id: HashMap<PublicKey<JubJub>, AccountId>,
}

impl State {
    /// Create an empty ledger that supports `num_accounts` accounts.
    pub fn new(num_accounts: usize, parameters: &Parameters) -> Self {
        let height = log2(num_accounts);
        let account_merkle_tree: AccMerkleTree = MerkleTree::blank(
            &parameters.leaf_crh_params,
            &parameters.two_to_one_crh_params,
            height as usize,
        )
        .unwrap();
        let pub_key_to_id = HashMap::with_capacity(num_accounts);
        let id_to_account_info = HashMap::with_capacity(num_accounts);
        Self {
            next_available_account: Some(AccountId(1)),
            account_merkle_tree,
            pub_key_to_id,
            id_to_account_info,
        }
    }

    /// Return the root of the account Merkle tree.
    pub fn root(&self) -> AccRoot {
        self.account_merkle_tree.root()
    }

    /// Create a new account with public key `pub_key`. Returns a fresh account identifier
    /// if there is space for a new account, and returns `None` otherwise.
    /// The initial balance of the new account is 0.
    pub fn register(&mut self, public_key: AccountPublicKey) -> Option<AccountId> {
        self.next_available_account.map(|id| {
            // Construct account information for the new account.
            let account_info = AccountInformation {
                public_key,
                nonce: Nonce(0),
            };
            // Insert information into the relevant accounts.
            self.pub_key_to_id.insert(public_key, id);
            let mut uncompressed_bytes = Vec::new();
            // todo: revisit unwrap()
            account_info
                .serialize_uncompressed(&mut uncompressed_bytes)
                .unwrap();
            self.account_merkle_tree
                .update(id.0 as usize, uncompressed_bytes.as_slice())
                .expect("should exist");
            self.id_to_account_info.insert(id, account_info);
            // Increment the next account identifier.
            self.next_available_account
                .as_mut()
                .and_then(|current_account| current_account.checked_increment());
            id
        })
    }

    /// Samples keys and registers these in the ledger.
    pub fn sample_keys_and_register<R: Rng>(
        &mut self,
        ledger_params: &Parameters,
        rng: &mut R,
    ) -> Option<(AccountId, AccountPublicKey, AccountSecretKey)> {
        let (pub_key, secret_key) =
            <Schnorr<JubJub, Blake2s> as SignatureScheme>::keygen(&ledger_params.sig_params, rng)
                .unwrap();
        self.register(pub_key).map(|id| (id, pub_key, secret_key))
    }

    /// Update the nonce of `id` to `new_nonce`.
    /// Returns `Some(())` if an account with identifier `id` exists already, and `None`
    /// otherwise.
    fn increment_nonce(&mut self, id: AccountId) -> Option<()> {
        let tree = &mut self.account_merkle_tree;
        self.id_to_account_info.get_mut(&id).map(|account_info| {
            account_info.nonce.checked_increment().unwrap();
            let mut uncompressed_bytes = Vec::new();
            account_info
                .serialize_uncompressed(&mut uncompressed_bytes)
                .unwrap();
            tree.update(id.0 as usize, uncompressed_bytes.as_slice())
                .expect("should exist")
        })
    }

    /// Update the state by applying the transaction `tx`, if `tx` is valid.
    pub fn apply_transaction(&mut self, pp: &Parameters, tx: &Transaction) -> Option<()> {
        if tx.validate(pp, self) {
            self.increment_nonce(tx.sender);
            Some(())
        } else {
            None
        }
    }

    pub fn get_account_info(&self, id: AccountId) -> Option<AccountInformation> {
        self.id_to_account_info
            .get(&id)
            .map(|account_info| AccountInformation {
                public_key: account_info.public_key,
                nonce: account_info.nonce,
            })
    }
}

#[test]
fn test_state_tree() {
    let mut rng = ark_std::test_rng();
    let pp = Parameters::sample(&mut rng);
    let mut state: State = State::new(32, &pp);
    let (alice_id, _alice_pk, alice_sk) = state.sample_keys_and_register(&pp, &mut rng).unwrap();
    let tx = Transaction::new(&pp, alice_id, Nonce(0), &alice_sk, &mut rng);
    assert!(tx.validate(&pp, &state));
    state.apply_transaction(&pp, &tx).expect("should work");
    // Let's try creating second "VALID" transaction
    let tx2 = Transaction::new(&pp, alice_id, Nonce(1), &alice_sk, &mut rng);
    assert!(tx2.validate(&pp, &state));
    state.apply_transaction(&pp, &tx2).expect("should work");
    // Let's try creating third "INVALID" transaction, incorrect nonce
    let bad_tx1 = Transaction::new(&pp, alice_id, Nonce(10), &alice_sk, &mut rng);
    assert!(!bad_tx1.validate(&pp, &state));
    assert!(matches!(state.apply_transaction(&pp, &bad_tx1), None));
    // Next, let's try a transaction where the signature is incorrect:
    // Let's make an account for Bob.
    let (_bob_id, _bob_pk, bob_sk) = state.sample_keys_and_register(&pp, &mut rng).unwrap();
    let bad_tx2 = Transaction::new(&pp, alice_id, Nonce(2), &bob_sk, &mut rng);
    assert!(!bad_tx2.validate(&pp, &state));
    assert!(matches!(state.apply_transaction(&pp, &bad_tx2), None));
    // Finally, let's try a good transaction
    let good_tx = Transaction::new(&pp, alice_id, Nonce(2), &alice_sk, &mut rng);
    assert!(good_tx.validate(&pp, &state));
}
