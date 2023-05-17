use crate::block::Hash;
use elliptic_curve::generic_array::GenericArray;
use k256::{
    ecdsa::{Signature, SigningKey, VerifyingKey},
    schnorr::signature::{Signer, Verifier},
};
use sha2::Digest;

#[derive(Debug)]
pub enum TransactionError {
    NoFromSignError,
    ForeignPubkey,
    NotLoan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionKind {
    Normal,
    Loan(Option<Vec<u8>>), // the other parties signiture
    Repayment,
}

impl TransactionKind {
    fn bytes(&self) -> &[u8] {
        match self {
            TransactionKind::Normal => &[0],
            TransactionKind::Loan(_) => &[1],
            TransactionKind::Repayment => &[2],
        }
    }

    fn is_loan(&self) -> bool {
        match self {
            TransactionKind::Loan(_) => true,
            _ => false,
        }
    }

    fn loan_signed(&self) -> bool {
        match self {
            TransactionKind::Loan(sig) => sig.is_some(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    from: Option<String>,
    to: String,
    amount: u64,
    hash: Hash,
    signiture: Vec<u8>,
    kind: TransactionKind,
}

impl Transaction {
    pub fn new(from: Option<String>, to: String, amount: u64, kind: TransactionKind) -> Self {
        let hash = Self::hash_transaction(&from, &to, amount, &kind);

        Self {
            from,
            to,
            amount,
            hash,
            signiture: Vec::new(),
            kind,
        }
    }

    pub fn loan_signed(&self) -> bool {
        self.valid() && self.kind.loan_signed()
    }

    pub fn is_loan(&self) -> bool {
        self.kind.is_loan()
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

    pub fn hash(&self) -> Hash {
        self.hash
    }

    pub fn kind(&self) -> &TransactionKind {
        &self.kind
    }

    fn hash_transaction(
        from: &Option<String>,
        to: &str,
        amount: u64,
        kind: &TransactionKind,
    ) -> Hash {
        let bytes = [
            match from {
                Some(f) => f.as_bytes(),
                None => &[0],
            },
            to.as_bytes(),
            &amount.to_be_bytes(),
            kind.bytes(),
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<u8>>();

        sha2::Sha256::digest(bytes).into()
    }

    pub fn sign_transaction(&mut self, private_key: &SigningKey) -> Result<(), TransactionError> {
        let Some(from) = self.from.as_ref() else {
            return Err(TransactionError::NoFromSignError)
        };

        let public_key = VerifyingKey::from(private_key);
        let public_key = serde_json::to_string(&public_key).unwrap();

        if &public_key != from {
            return Err(TransactionError::ForeignPubkey);
        }

        let signiture: Signature = private_key.sign(&self.hash);
        self.signiture = signiture.to_bytes().to_vec();

        Ok(())
    }

    pub fn sign_loan_transaction(
        &mut self,
        private_key: &SigningKey,
    ) -> Result<(), TransactionError> {
        if !self.is_loan() {
            return Err(TransactionError::NotLoan);
        }

        let public_key = VerifyingKey::from(private_key);
        let public_key = serde_json::to_string(&public_key).unwrap();

        if self.to != public_key {
            return Err(TransactionError::ForeignPubkey);
        }

        let signiture: Signature = private_key.sign(&self.hash);
        self.kind = TransactionKind::Loan(Some(signiture.to_bytes().to_vec()));

        Ok(())
    }

    pub fn valid(&self) -> bool {
        let Some(from) = self.from.as_ref() else {
            return true;
        };

        if self.signiture.is_empty() {
            return false;
        }

        let public_key: VerifyingKey = serde_json::from_str(from).unwrap();
        let signiture = Signature::from_bytes(GenericArray::from_slice(&self.signiture)).unwrap();
        public_key.verify(&self.hash, &signiture).is_ok()
    }
}
