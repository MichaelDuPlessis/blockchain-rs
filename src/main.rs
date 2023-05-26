mod block;
mod blockchain;
mod keygen;
mod transaction;

use crate::transaction::TransactionKind;
use blockchain::Blockchain;
use k256::ecdsa::{SigningKey, VerifyingKey};
use std::collections::HashMap;
use text_io::read;
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::default();
    let mut users: HashMap<String, (SigningKey, VerifyingKey)> = HashMap::new();

    loop {
        println!("Enter a command: (list, pay, add, loan, info, mine, sign, print, exit)");
        let input: String = read!("{}\n");

        match input.to_lowercase().as_str() {
            "list" => list(&users),
            "pay" => pay(&users, &mut blockchain),
            "add" => loan(&users, &mut blockchain),
            "loan" => add(&mut users),
            "info" => info(&users, &blockchain),
            "mine" => mine(&users, &mut blockchain),
            "print" => print_blockchain(&blockchain),
            "exit" => sign_loan(&users, &mut blockchain),
            "sign" => break,
            _ => println!("Unknown command."),
        }

        println!();
    }
}

fn print_blockchain(blockchain: &Blockchain) {
    for block in blockchain.blocks() {
        println!(
            "Prev hash: {:X?}\nHash: {:X?}",
            block.prev_hash(),
            block.hash()
        )
    }
}

fn list(users: &HashMap<String, (SigningKey, VerifyingKey)>) {
    println!("Users");
    for (name, address) in users {
        println!("{} {}", name, serde_json::to_string(&address.1).unwrap());
    }
}

fn pay(users: &HashMap<String, (SigningKey, VerifyingKey)>, blockchain: &mut Blockchain) {
    println!("Who is paying");
    let input: String = read!("{}\n");
    let payer = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    println!("Who is being paid");
    let input: String = read!("{}\n");
    let payee = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    println!("Enter an amount to pay:");
    let amount: u64 = read!("{}\n");
    // let amount = amount.parse::<u64>().unwrap();

    let mut transaction = Transaction::new(
        Some(serde_json::to_string(&payer.1).unwrap()),
        serde_json::to_string(&payee.1).unwrap(),
        amount,
        TransactionKind::Normal,
    );

    transaction.sign_transaction(&payer.0).unwrap();
    blockchain.add_transaction(transaction).unwrap();
}

fn add(users: &mut HashMap<String, (SigningKey, VerifyingKey)>) {
    println!("Enter a username:");
    let input: String = read!("{}\n");

    let address = keygen::gen_key_pair();

    users.insert(input, address);
}

fn mine(users: &HashMap<String, (SigningKey, VerifyingKey)>, blockchain: &mut Blockchain) {
    println!("Enter a username:");
    let input: String = read!("{}\n");
    let user = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    blockchain.mine_pending_transactions(serde_json::to_string(&user.1).unwrap());
}

fn info(users: &HashMap<String, (SigningKey, VerifyingKey)>, blockchain: &Blockchain) {
    println!("Enter a username:");
    let input: String = read!("{}\n");
    let user = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    println!(
        "Balance: {}\n",
        blockchain
            .balance_of(&serde_json::to_string(&user.1).unwrap())
            .unwrap()
    );

    println!("Signed Loans:");
    let loans = blockchain.loans_of(&serde_json::to_string(&user.1).unwrap(), true);
    for (hash, (to, amount)) in loans {
        println!("Hash: {:X?} Amount: {} To: {}", hash, amount, to);
    }
    let loans = blockchain.all_loans_of(&serde_json::to_string(&user.1).unwrap());
    for (hash, (to, amount)) in loans {
        println!("Hash: {:X?} Amount: {} To: {}", hash, amount, to);
    }

    println!();

    println!("Unsigned Loans:");
    let loans = blockchain.loans_of(&serde_json::to_string(&user.1).unwrap(), false);
    for (hash, (to, amount)) in loans {
        println!("Hash: {:X?} Amount: {} To: {}", hash, amount, to);
    }
}

fn loan(users: &HashMap<String, (SigningKey, VerifyingKey)>, blockchain: &mut Blockchain) {
    println!("Who is loaing");
    let input: String = read!("{}\n");
    let payer = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    println!("Who is loaner");
    let input: String = read!("{}\n");
    let payee = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    println!("Enter an amount to loan:");
    let amount: u64 = read!("{}\n");

    let mut transaction = Transaction::new(
        Some(serde_json::to_string(&payer.1).unwrap()),
        serde_json::to_string(&payee.1).unwrap(),
        amount,
        TransactionKind::Loan(None),
    );

    transaction.sign_transaction(&payer.0).unwrap();
    blockchain.add_transaction(transaction).unwrap();
}

fn sign_loan(users: &HashMap<String, (SigningKey, VerifyingKey)>, blockchain: &mut Blockchain) {
    println!("Who is signing");
    let input: String = read!("{}\n");
    let user = if let Some(user) = users.get(&input) {
        user
    } else {
        println!("No user found.");
        return;
    };

    println!("Unsigned Loans:");
    let loans = blockchain.loans_of(&serde_json::to_string(&user.1).unwrap(), false);
    let transactions = loans.iter().collect::<Vec<_>>();
    for (i, (hash, (to, amount))) in transactions.iter().enumerate() {
        println!("{i}. Hash: {:X?} Amount: {} To: {}", hash, amount, to);
    }
    println!("\nEnter a number to sign");
    let pos: usize = read!("{}\n");
    match blockchain.sign_loan(&user.0, *transactions[pos].0) {
        Ok(_) => (),
        Err(e) => match e {
            blockchain::BlockchainError::InvalidTransaction => todo!(),
            blockchain::BlockchainError::NegativeBalance => todo!(),
            blockchain::BlockchainError::BalanceTooSmall => todo!(),
            blockchain::BlockchainError::InvalidSigner => {
                println!("Failed: User is not involved in this loan")
            }
            blockchain::BlockchainError::NoTransactionFound => {
                println!("Failed: Transaction not found")
            }
        },
    }
}
