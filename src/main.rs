use transaction::Transaction;

mod block;
mod blockchain;
mod transaction;

fn main() {
    let mut blockchain = blockchain::Blockchain::default();

    blockchain.add_transaction(Transaction::new("A".to_string(), "B".to_string(), 100));
    blockchain.add_transaction(Transaction::new("A".to_string(), "B".to_string(), 50));
    blockchain.add_transaction(Transaction::new("B".to_string(), "A".to_string(), 25));

    blockchain.mine_pending_transactions("B");

    for block in blockchain.blocks() {
        println!("prev_hash: {:x?}", block.prev_hash());
        println!("hash: {:x?}", block.hash());
    }

    println!("Balance: {}", blockchain.balance_of("B").unwrap());

    blockchain.mine_pending_transactions("B");

    for block in blockchain.blocks() {
        println!("prev_hash: {:x?}", block.prev_hash());
        println!("hash: {:x?}", block.hash());
    }

    println!("Balance: {}", blockchain.balance_of("B").unwrap());
}
