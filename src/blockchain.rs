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
        &self
            .blocks
            .last()
            .expect("There should always be a latest block")
    }

    pub fn mine_pending_transactions(&mut self, reward_address: impl Into<String>) {
        let mut transactions = vec![Transaction::new(
            String::new(),
            reward_address.into(),
            REWARD,
        )]; // creating new mempool with reward
        std::mem::swap(&mut transactions, &mut self.mempool);

        // block is mined when it is created
        let block = Block::new(transactions, self.latest_block().hash());
        self.blocks.push(block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
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
                    .into_iter()
                    .filter_map(|transaction| {
                        if transaction.from() == address {
                            Some(-(transaction.amount() as i64))
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
            Err(())
        } else {
            Ok(balance as u64)
        }
    }
}
