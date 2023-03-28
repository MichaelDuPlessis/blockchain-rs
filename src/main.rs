mod block;
mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::default();

    blockchain.add_block(block::Block::new(1, b"Testing 1".to_vec()));
    blockchain.add_block(block::Block::new(1, b"Testing 2".to_vec()));
    blockchain.add_block(block::Block::new(1, b"Testing 3".to_vec()));

    for block in blockchain.blocks() {
        println!("prev_hash: {:x?}", block.prev_hash().unwrap());
        println!("hash: {:x?}", block.hash().unwrap());
        println!("data: {}", std::str::from_utf8(block.data()).unwrap());
    }
}
