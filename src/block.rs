use sha2::Digest;
use std::time::SystemTime;

use crate::transaction::Transaction;

const DIFFICULTY: usize = 2; // how many bytes need to be -
const DIFFICULTY_TEST: [u8; DIFFICULTY] = [0; DIFFICULTY];

// alias for the size of a hash
pub type Hash = [u8; 32];

// A block of a blockchain
pub struct Block {
    timestamp: u64,
    transactions: Vec<Transaction>,
    prev_hash: Hash,
    hash: Hash, // as the hash may not be calculated yet
    nonce: u64,
}

impl Block {
    // creates a new block without the prev_hash
    pub fn new(transactions: impl Into<Vec<Transaction>>, prev_hash: Hash) -> Self {
        let transactions = transactions.into();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Something went wrong getting current time")
            .as_secs();

        // mining the block and getting the nonce
        let mut nonce = 0;
        let hash = loop {
            let hash = Self::calculate_hash(timestamp, nonce, &transactions, &prev_hash);
            if hash[..DIFFICULTY] == DIFFICULTY_TEST {
                break hash;
            }

            nonce += 1;
        };

        Self {
            timestamp,
            transactions,
            prev_hash,
            hash,
            nonce,
        }
    }

    pub fn valid_transactions(&self) -> bool {
        for transaction in &self.transactions {
            if !transaction.valid() {
                return false;
            }
        }

        true
    }

    // the function to calculate a hash
    pub fn calculate_hash(
        timestamp: u64,
        nonce: u64,
        transactions: &[Transaction],
        prev_hash: &[u8],
    ) -> Hash {
        // this code nees to be fixed later
        let bytes: Vec<_> = [
            prev_hash,
            &timestamp.to_be_bytes(),
            &nonce.to_be_bytes(),
            &transactions
                .iter()
                .flat_map(|transaction| transaction.hash())
                .collect::<Vec<u8>>(),
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect();

        sha2::Sha256::digest(bytes).into()
    }

    // gets the previous hash
    pub fn hash(&self) -> Hash {
        self.hash
    }

    // gets the previous hash
    pub fn prev_hash(&self) -> Hash {
        self.prev_hash
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
