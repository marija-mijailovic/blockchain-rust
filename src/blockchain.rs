use std::collections::HashMap;

use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub accounts: HashMap<String, u32>
}

impl Blockchain {

  pub fn add_accounts(&mut self, account: String) {
    self.accounts.insert(account, 100);
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    println!("Accounts {:?}", self.accounts);
    let tx = transaction.clone();
    let transactions = vec![transaction];
    match self.blocks.last() {
      Some(prev_block) => {
        let mut block = Block::new(transactions, &prev_block);
        block.set_hash();
        self.blocks.push(block);
        let new_sender_balance = match self.accounts.get(&tx.from) {
          Some(balance) => {
            if *balance > tx.value {
              let balance_after_tx = *balance - tx.value;
              balance_after_tx
            } else {
              panic!("Sender {:?} balance is invalid", tx.from);
            }
          },
          None => panic!("Sender {:?} balance is invalid", tx.from)
        };
        self.accounts.insert(tx.from, new_sender_balance);
        
        let receiver_balance = match self.accounts.get(&tx.to) {
          Some(balance) => balance + tx.value,
          None => tx.value
        };
        self.accounts.insert(tx.to, receiver_balance);
      },
      None => println!("There is no previous block")
    }
  }
}