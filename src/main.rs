use std::collections::HashMap;

use blockchain_rust::block::Block;
use blockchain_rust::transaction::Transaction;
use blockchain_rust::blockchain::Blockchain;

fn main() {
    let genesis_block = Block::genesis();
    let mut accounts = HashMap::new();
    accounts.insert("Alice".to_owned(), 120);
    accounts.insert("Davide".to_owned(), 120);
    
    let mut blockchain = Blockchain {
        blocks: vec![genesis_block],
        accounts: accounts
    };
    
    let tx1 = Transaction::new("Alice".to_owned(), "Bob".to_owned(), 10);
    
    blockchain.add_transaction(tx1);
    
    let tx2 = Transaction::new("Davide".to_owned(), "Charlie".to_owned(), 17);
    
    blockchain.add_transaction(tx2);

    let tx3 = Transaction::new("Charlie".to_owned(), "Alice".to_owned(), 4);
    
    blockchain.add_transaction(tx3);
    println!("{:?}",blockchain);
}
