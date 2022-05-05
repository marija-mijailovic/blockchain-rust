use std::collections::HashMap;
use clap::{arg, Command};

use blockchain_rust::block::Block;
use blockchain_rust::transaction::Transaction;
use blockchain_rust::blockchain::Blockchain;

fn main() {
    let mut accounts = HashMap::new();
    accounts.insert("Alice".to_owned(), 100);
    let genesis_block = Block::genesis();
    let mut blockchain = Blockchain {
        blocks: vec![genesis_block],
        accounts: accounts
    };

    let matches = Command::new("blockchain-rust").
        author("Marija Mijailovic").
        about("Simple blockchain in rust for education purpose").
        arg_required_else_help(true).
        subcommand(
            Command::new("create-account").
                about("Create account for transactions").
                arg(arg!(<account> "Account for transaction")).
                arg_required_else_help(true)).
        subcommand(
            Command::new("create-transaction").
                about("Create transactions").
                arg(arg!(<from> "Transaction from")).
                arg(arg!(<to> "Transaction to")).
                arg(arg!(<amount> "Transaction amount")).
                arg_required_else_help(true)).
        get_matches();

    if let Some(matches) = matches.subcommand_matches("create-account") {
        if let Some(account) = matches.value_of("account") {
            blockchain.add_accounts(account.to_owned());
        } else {
            println!("Invalid account command");
        }
    } else if let Some(matches) = matches.subcommand_matches("create-transaction") {
        if let Some(from) = matches.value_of("from") {
            if let Some(to) = matches.value_of("to") {
                if let Some(amount) = matches.value_of("amount") {
                    let tx1 = Transaction::new(from.to_owned(), to.to_owned(), amount.to_owned().parse().unwrap());
                    blockchain.add_transaction(tx1);
                } else {
                    println!("Invalid amount command");
                }
            } else {
                println!("Invalid to command");
            }
        } else {
            println!("Invalid from command");
        }
    }
    
    println!("{:?}",blockchain);
}
