use core::panic;

use crate::block::Hash;
use elliptic_curve::{generic_array::GenericArray, SecretKey};
use k256::{
    ecdsa::{Signature, SigningKey, VerifyingKey},
    schnorr::signature::{Signer, Verifier},
};
use sha2::Digest;

pub struct Transaction {
    from: Option<String>,
    to: String,
    amount: u64,
    hash: Hash,
    signiture: Vec<u8>,
}

impl Transaction {
    pub fn new(from: Option<String>, to: String, amount: u64) -> Self {
        let hash = Self::hash(&from, &to, amount);

        Self {
            from,
            to,
            amount,
            hash,
            signiture: Vec::new(),
        }
    }

    pub fn from(&self) -> &Option<String> {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn signiture_hash(&self) -> Hash {
        self.hash
    }

    fn hash(from: &Option<String>, to: &str, amount: u64) -> Hash {
        let bytes = [
            match from {
                Some(f) => f.as_bytes(),
                None => &[0],
            },
            to.as_bytes(),
            &amount.to_be_bytes(),
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<u8>>();

        sha2::Sha256::digest(bytes).into()
    }

    pub fn sign_transaction(&mut self, private_key: &SigningKey) {
        let Some(from) = self.from.as_ref() else {
            panic!("Cannot sign a transaction from none.")
        };

        let public_key = VerifyingKey::from(private_key);
        let public_key = serde_json::to_string(&public_key).unwrap();

        if &public_key != from {
            panic!("Cannot sign transaction for foreign public key");
        }

        let signiture: Signature = private_key.sign(&self.hash);
        self.signiture = signiture.to_bytes().to_vec();
    }

    pub fn valid(&self) -> bool {
        let Some(from) = self.from.as_ref() else {
            return true;
        };

        if self.signiture.is_empty() {
            panic!("Signature is empty")
        }

        let public_key: VerifyingKey = serde_json::from_str(&from).unwrap();
        let signiture = Signature::from_bytes(GenericArray::from_slice(&self.signiture)).unwrap();
        public_key.verify(&self.hash, &signiture).is_ok()
    }
}
