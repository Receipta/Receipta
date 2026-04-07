#![no_std]

mod types;
pub use types::{DataKey, FeeConfig, Receipt, ReceiptError, ReceiptStatus};

use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env};

/// Generates a deterministic 32-byte receipt ID by hashing
/// (sender, receiver, amount, timestamp) with SHA-256.
pub fn generate_receipt_id(
    env: &Env,
    sender: &Address,
    receiver: &Address,
    amount: i128,
    timestamp: u64,
) -> BytesN<32> {
    let mut data = Bytes::new(env);

    // Encode sender address bytes
    let sender_bytes = sender.to_xdr(env);
    data.append(&sender_bytes);

    // Encode receiver address bytes
    let receiver_bytes = receiver.to_xdr(env);
    data.append(&receiver_bytes);

    // Encode amount as 16 big-endian bytes
    let amount_bytes: [u8; 16] = amount.to_be_bytes();
    data.append(&Bytes::from_array(env, &amount_bytes));

    // Encode timestamp as 8 big-endian bytes
    let ts_bytes: [u8; 8] = timestamp.to_be_bytes();
    data.append(&Bytes::from_array(env, &ts_bytes));

    env.crypto().sha256(&data)
}

#[contract]
pub struct ReceiptaContract;

#[contractimpl]
impl ReceiptaContract {
    // Core receipt functions will be implemented in task 3
    // Status/fee functions will be implemented in task 4
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_receipt_id_determinism() {
        let env = Env::default();
        let sender = Address::generate(&env);
        let receiver = Address::generate(&env);
        let amount: i128 = 1_000_000;
        let timestamp: u64 = 1_700_000_000;

        let id1 = generate_receipt_id(&env, &sender, &receiver, amount, timestamp);
        let id2 = generate_receipt_id(&env, &sender, &receiver, amount, timestamp);

        assert_eq!(id1, id2, "same inputs must produce the same receipt ID");
    }

    #[test]
    fn test_receipt_id_different_amounts() {
        let env = Env::default();
        let sender = Address::generate(&env);
        let receiver = Address::generate(&env);
        let timestamp: u64 = 1_700_000_000;

        let id1 = generate_receipt_id(&env, &sender, &receiver, 1_000_000, timestamp);
        let id2 = generate_receipt_id(&env, &sender, &receiver, 2_000_000, timestamp);

        assert_ne!(id1, id2, "different amounts must produce different receipt IDs");
    }

    #[test]
    fn test_receipt_id_different_timestamps() {
        let env = Env::default();
        let sender = Address::generate(&env);
        let receiver = Address::generate(&env);
        let amount: i128 = 1_000_000;

        let id1 = generate_receipt_id(&env, &sender, &receiver, amount, 1_700_000_000);
        let id2 = generate_receipt_id(&env, &sender, &receiver, amount, 1_700_000_001);

        assert_ne!(id1, id2, "different timestamps must produce different receipt IDs");
    }

    #[test]
    fn test_receipt_id_different_senders() {
        let env = Env::default();
        let sender1 = Address::generate(&env);
        let sender2 = Address::generate(&env);
        let receiver = Address::generate(&env);
        let amount: i128 = 1_000_000;
        let timestamp: u64 = 1_700_000_000;

        let id1 = generate_receipt_id(&env, &sender1, &receiver, amount, timestamp);
        let id2 = generate_receipt_id(&env, &sender2, &receiver, amount, timestamp);

        assert_ne!(id1, id2, "different senders must produce different receipt IDs");
    }

    #[test]
    fn test_receipt_id_different_receivers() {
        let env = Env::default();
        let sender = Address::generate(&env);
        let receiver1 = Address::generate(&env);
        let receiver2 = Address::generate(&env);
        let amount: i128 = 1_000_000;
        let timestamp: u64 = 1_700_000_000;

        let id1 = generate_receipt_id(&env, &sender, &receiver1, amount, timestamp);
        let id2 = generate_receipt_id(&env, &sender, &receiver2, amount, timestamp);

        assert_ne!(id1, id2, "different receivers must produce different receipt IDs");
    }

    #[test]
    fn test_receipt_id_sender_receiver_swap_differs() {
        // Ensures hash(A, B, ...) != hash(B, A, ...) — order matters
        let env = Env::default();
        let addr_a = Address::generate(&env);
        let addr_b = Address::generate(&env);
        let amount: i128 = 1_000_000;
        let timestamp: u64 = 1_700_000_000;

        let id1 = generate_receipt_id(&env, &addr_a, &addr_b, amount, timestamp);
        let id2 = generate_receipt_id(&env, &addr_b, &addr_a, amount, timestamp);

        assert_ne!(id1, id2, "swapping sender and receiver must produce different IDs");
    }
}
