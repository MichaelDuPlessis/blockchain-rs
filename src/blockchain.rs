use crate::{
    block::Block,
    transaction::{self, Transaction, TransactionKind},
};
use indexmap::IndexMap;
use k256::ecdsa::SigningKey;

const REWARD: u64 = 1000; // for now just constant

#[derive(Debug)]
pub enum BlockchainError {
    InvalidTransaction,
    NegativeBalance,
    BalanceTooSmall,
    InvalidSigner,
    NoTransactionFound,
}

// The actual blockchain
pub struct Blockchain {
    blocks: Vec<Block>,
    mempool: Vec<Transaction>,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self {
            blocks: vec![Block::new(Vec::new(), [0; 32])], // creating the genesis block
            mempool: Vec::new(),
        }
    }
}

impl Blockchain {
    // get a reference to the vec of blocks
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    // getting the last block from the chain
    pub fn latest_block(&self) -> &Block {
        self.blocks
            .last()
            .expect("There should always be a latest block")
    }

    pub fn mine_pending_transactions(&mut self, reward_address: impl Into<String>) {
        let mut transactions = vec![Transaction::new(
            None,
            reward_address.into(),
            REWARD,
            TransactionKind::Normal,
        )]; // creating new mempool with reward

        let mut to_remove = Vec::new();
        for (i, transaction) in self.mempool.iter().enumerate() {
            let loans = self.all_loans_of(transaction.to());
            let mut amount_recieved = transaction.amount();

            for (_, (to, amount)) in &loans {
                if let Some(remaining) = amount_recieved.checked_sub(*amount) {
                    amount_recieved = remaining;
                    transactions.push(Transaction::new(
                        Some(transaction.to().to_owned()),
                        (*to).to_owned(),
                        *amount,
                        TransactionKind::Repayment,
                    ));
                } else {
                    break;
                }
            }

            // if there is a loan but it has not been signed by both parties then it must not be
            // mined
            if transaction.is_loan() && !transaction.loan_signed() {
                to_remove.push(i)
            }
        }

        // moving transavtions back to mempool
        for i in to_remove {
            transactions.push(self.mempool.remove(i))
        }

        std::mem::swap(&mut transactions, &mut self.mempool);

        // block is mined when it is created
        let block = Block::new(transactions, self.latest_block().hash());
        self.blocks.push(block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), BlockchainError> {
        if !transaction.valid() {
            return Err(BlockchainError::InvalidTransaction);
        }

        if let Ok(balance) = self.balance_of(transaction.from().as_ref().unwrap()) {
            // making sure user has enough to pay
            if transaction.amount() > balance {
                return Err(BlockchainError::BalanceTooSmall);
            }
        } else {
            return Err(BlockchainError::BalanceTooSmall);
        }

        self.mempool.push(transaction);
        Ok(())
    }

    // if balance is negative transaction cannot be made
    pub fn balance_of(&self, address: &str) -> Result<u64, BlockchainError> {
        let balance = self
            .blocks
            .iter()
            .map(|block| {
                block
                    .transactions()
                    .iter()
                    .filter_map(|transaction| {
                        if let Some(from) = transaction.from() {
                            if from == address {
                                Some(-(transaction.amount() as i64))
                            } else if transaction.to() == address {
                                Some(transaction.amount() as i64)
                            } else {
                                None
                            }
                        } else if transaction.to() == address {
                            Some(transaction.amount() as i64)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>()
            })
            .sum::<i64>();

        if balance.is_negative() {
            Err(BlockchainError::NegativeBalance)
        } else {
            Ok(balance as u64)
        }
    }

    // for all loans returns the address and amount owed to an address
    // looks in blockchain
    pub fn all_loans_of(&self, address: &str) -> IndexMap<[u8; 32], (&str, u64)> {
        let mut loans = IndexMap::new();

        for block in self.blocks() {
            for transaction in block.transactions() {
                if transaction.is_loan() {
                    if transaction.to() == address {
                        loans.insert(transaction.hash(), (transaction.to(), transaction.amount()));
                    }
                }
            }
        }

        loans
    }

    // returns the loans of a user
    // looks in mempool
    pub fn loans_of(&self, address: &str, valid: bool) -> IndexMap<[u8; 32], (&str, u64)> {
        let mut loans = IndexMap::new();

        for transaction in &self.mempool {
            if transaction.is_loan() && transaction.loan_signed() == valid {
                if transaction.to() == address {
                    loans.insert(transaction.hash(), (transaction.to(), transaction.amount()));
                }
            }
        }

        loans
    }

    pub fn sign_loan(
        &mut self,
        payee: &SigningKey,
        transaction_hash: [u8; 32],
    ) -> Result<(), BlockchainError> {
        let Some(transaction) = self
            .mempool
            .iter_mut()
            .find(|transaction| transaction.hash() == transaction_hash) else {
                return Err(BlockchainError::NoTransactionFound); 
            };

        transaction.sign_loan_transaction(payee);

        Ok(())
    }

    pub fn paid_to(&self, from: &str, to: &str) -> u64 {
        self.blocks()
            .iter()
            .map(|block| {
                block
                    .transactions()
                    .iter()
                    .filter_map(|transaction| {
                        if let Some(f) = transaction.from().as_ref() {
                            if f == from && transaction.to() == to {
                                Some(transaction.amount())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    pub fn total_loan_cost(&self, from: &str, to: &str) -> u64 {
        self.blocks()
            .iter()
            .map(|block| {
                block
                    .transactions()
                    .iter()
                    .filter(|transaction| transaction.is_loan())
                    .filter_map(|transaction| {
                        if let Some(f) = transaction.from().as_ref() {
                            if f == from && transaction.to() == to {
                                Some(transaction.amount())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .sum::<u64>()
            })
            .sum()
    }

    fn valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let cur_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            if !cur_block.valid_transactions() {
                return false;
            }

            if cur_block.hash()
                != Block::calculate_hash(
                    cur_block.timestamp(),
                    cur_block.nonce(),
                    cur_block.transactions(),
                    &cur_block.prev_hash(),
                )
            {
                return false;
            }

            if cur_block.prev_hash() != prev_block.hash() {
                return false;
            }
        }

        true
    }
}
