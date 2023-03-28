use core::panic;
use sha2::Digest;
use std::time::SystemTime;

const DIFFICULTY: usize = 2; // how many bytes need to be -
const DIFFICULTY_TEST: [u8; DIFFICULTY] = [0; DIFFICULTY];

// alias for the size of a hash
type Hash = [u8; 32];

// the function to calculate a hash
fn calculate_hash(index: u64, timestamp: u64, nonce: u64, data: &[u8], prev_hash: &[u8]) -> Hash {
    let bytes: Vec<_> = [
        &index.to_be_bytes(),
        &timestamp.to_be_bytes(),
        &nonce.to_be_bytes(),
        data,
        prev_hash,
    ]
    .into_iter()
    .flatten()
    .map(|b| *b)
    .collect();

    sha2::Sha256::digest(bytes).into()
}

// A block of a blockchain
pub struct Block {
    index: usize,
    timestamp: u64,
    data: Vec<u8>,
    prev_hash: Option<Hash>,
    hash: Option<Hash>, // as the hash may not be calculated yet
    nonce: u64,
}

impl Block {
    pub fn new_with_prev_hash(index: usize, data: impl Into<Vec<u8>>, prev_hash: Hash) -> Self {
        let data = data.into();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Something went wrong getting current time")
            .as_secs();

        let mut block = Self {
            index,
            timestamp,
            data,
            prev_hash: Some(prev_hash),
            hash: None,
            nonce: 0, // set to 0 for now
        };

        block.mine();

        block
    }

    // creates a new block without the prev_hash
    pub fn new(index: usize, data: impl Into<Vec<u8>>) -> Self {
        let data = data.into();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Something went wrong getting current time")
            .as_secs();

        Self {
            index,
            timestamp,
            data,
            prev_hash: None,
            hash: None,
            nonce: 0,
        }
    }

    // sets the hash and mines the block
    pub fn set_prev_hash(&mut self, prev_hash: Hash) {
        self.prev_hash = Some(prev_hash);
        self.mine();
    }

    // gets the previous hash
    pub fn hash(&self) -> Option<Hash> {
        self.hash
    }

    // gets the previous hash
    pub fn prev_hash(&self) -> Option<Hash> {
        self.prev_hash
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn index(&self) -> usize {
        self.index
    }

    // used to mine the block and set nonce and hash
    fn mine(&mut self) {
        for nonce in 0.. {
            self.nonce = nonce; // setting nonce to be tested
            let hash = self.calculate_hash();

            if hash[..DIFFICULTY] == DIFFICULTY_TEST {
                self.hash = Some(hash);
                return;
            }
        }

        panic!("Unable to get here")
    }

    // the function to calculate a hash
    fn calculate_hash(&self) -> Hash {
        let Some(prev_hash) = &self.prev_hash else {
            panic!("No prev_hash set")
        };

        let index_bytes: &[u8] = &self.index.to_be_bytes();
        let timestamp_bytes: &[u8] = &self.timestamp.to_be_bytes();
        let nonce_bytes: &[u8] = &self.nonce.to_be_bytes();

        let bytes: Vec<_> = [
            index_bytes,
            timestamp_bytes,
            nonce_bytes,
            &self.data,
            prev_hash,
        ]
        .into_iter()
        .flatten()
        .map(|b| *b)
        .collect();

        sha2::Sha256::digest(bytes).into()
    }
}
