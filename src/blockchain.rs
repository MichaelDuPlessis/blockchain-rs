use crate::block::Block;

// The actual blockchain
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self {
            blocks: vec![Block::new_with_prev_hash(
                0,
                b"Genesis block".to_vec(),
                [0; 32],
            )], // creating the genesis block
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

    pub fn add_block(&mut self, mut block: Block) {
        let prev_hash = self
            .latest_block()
            .hash()
            .expect("All blocks in chain must have a previous hash");
        block.set_prev_hash(prev_hash);

        self.blocks.push(block);
    }
}
