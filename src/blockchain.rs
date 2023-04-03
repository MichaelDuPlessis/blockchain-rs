use crate::{block::Block, transaction::Transaction};

const REWARD: u64 = 10; // for now just constant

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
        let mut transactions = vec![Transaction::new(None, reward_address.into(), REWARD)]; // creating new mempool with reward

        std::mem::swap(&mut transactions, &mut self.mempool);

        // block is mined when it is created
        let block = Block::new(transactions, self.latest_block().hash());
        self.blocks.push(block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if !transaction.valid() {
            panic!("Invalid transaction cannot be added");
        }

        self.mempool.push(transaction);
    }

    // if balance is negative transaction cannot be maid
    pub fn balance_of(&self, address: &str) -> Result<u64, ()> {
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
                        } else {
                            if transaction.to() == address {
                                Some(transaction.amount() as i64)
                            } else {
                                None
                            }
                        }
                    })
                    .sum::<i64>()
            })
            .sum::<i64>();

        if balance.is_negative() {
            Err(())
        } else {
            Ok(balance as u64)
        }
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
