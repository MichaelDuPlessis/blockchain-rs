mod block;
mod blockchain;
mod keygen;
mod transaction;

use transaction::Transaction;

fn main() {
    let my_address = keygen::gen_key_pair();
    let other_address = keygen::gen_key_pair();

    let mut t1 = Transaction::new(
        Some(serde_json::to_string(&my_address.1).unwrap()),
        serde_json::to_string(&other_address.1).unwrap(),
        100,
        false,
    );
    let mut t2 = Transaction::new(
        Some(serde_json::to_string(&my_address.1).unwrap()),
        serde_json::to_string(&other_address.1).unwrap(),
        50,
        false,
    );
    let mut t3 = Transaction::new(
        Some(serde_json::to_string(&other_address.1).unwrap()),
        serde_json::to_string(&my_address.1).unwrap(),
        25,
        false,
    );

    t1.sign_transaction(&my_address.0).unwrap();
    t2.sign_transaction(&my_address.0).unwrap();
    t3.sign_transaction(&other_address.0).unwrap();

    let mut blockchain = blockchain::Blockchain::default();

    blockchain.add_transaction(t1).unwrap();
    blockchain.add_transaction(t2).unwrap();
    blockchain.add_transaction(t3).unwrap();

    blockchain.mine_pending_transactions(serde_json::to_string(&other_address.1).unwrap());

    for block in blockchain.blocks() {
        println!("prev_hash: {:x?}", block.prev_hash());
        println!("hash: {:x?}", block.hash());
    }

    println!(
        "Balance: {}",
        blockchain
            .balance_of(&serde_json::to_string(&other_address.1).unwrap())
            .unwrap()
    );

    println!("============================================");

    blockchain.mine_pending_transactions(serde_json::to_string(&other_address.1).unwrap());

    for block in blockchain.blocks() {
        println!("prev_hash: {:x?}", block.prev_hash());
        println!("hash: {:x?}", block.hash());
    }

    println!(
        "Balance: {}",
        blockchain
            .balance_of(&serde_json::to_string(&other_address.1).unwrap())
            .unwrap()
    );
}
