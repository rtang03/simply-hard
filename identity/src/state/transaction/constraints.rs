use crate::state::account::constraints::{AccountIdVar, NonceVar};

/// Transaction transferring some amount from one account to another.
pub struct TransactionVar {
    /// The account information of the sender.
    pub sender: AccountIdVar,
    /// The amount being transferred from the sender to the receiver.
    pub amount: NonceVar,
    // The spend authorization is a signature over the sender, the recipient,
    // and the amount.
    // pub signature: SignatureVar<EdwardsProjective, EdwardsVar>,
}
