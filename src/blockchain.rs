use crate::{
    block::Block,
    transaction::{self, Transaction, TransactionKind},
};
use indexmap::IndexMap;

const REWARD: u64 = 1000; // for now just constant

#[derive(Debug)]
pub enum BlockchainError {
    InvalidTransaction,
    NegativeBalance,
    BalanceTooSmall,
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

        for transaction in &self.mempool {
            let loans = self.loans_of(transaction.to());
            let mut amount_recieved = transaction.amount();

            for (to, amount) in &loans {
                if let Some(remaining) = amount_recieved.checked_sub(*amount) {
                    amount_recieved = remaining;
                    transactions.push(Transaction::new(
                        Some(transaction.to().to_owned()),
                        to.to_owned().to_owned(),
                        *amount,
                        TransactionKind::Repayment,
                    ));
                } else {
                    break;
                }
            }
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

    // returns the address and amount owed to an address
    pub fn loans_of(&self, address: &str) -> IndexMap<&str, u64> {
        let mut loans = IndexMap::new();

        for block in self.blocks() {
            for transaction in block.transactions() {
                if transaction.is_loan() {
                    if transaction.from().as_ref().unwrap() == address {
                        if let Some(loan) = loans.get_mut(transaction.to()) {
                            *loan += transaction.amount();
                        } else {
                            loans.insert(transaction.to(), transaction.amount());
                        }
                    }
                }
            }
        }

        loans
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
