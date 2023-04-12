use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub accounts: HashMap<String, u64>
}

impl Blockchain {
  pub fn get() -> Self {
    Blockchain {
      blocks: Self::get_blocks(),
      accounts: Self::get_accounts()
    }
  }

  pub fn add_accounts(&mut self, account: String) {
    self.accounts.entry(account).or_insert(100);
  }

  pub fn add_transaction(&mut self, transaction: Transaction) {
    let tx = transaction.clone();
    let transactions = vec![transaction];
    match self.blocks.last() {
      Some(prev_block) => {
        let mut block = Block::new(transactions, prev_block);
        block.set_hash();
        self.blocks.push(block);
        let new_sender_balance = match self.accounts.get(&tx.from) {
          Some(balance) => {
            if *balance > tx.value {
              *balance - tx.value
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

  pub fn account_balance(&mut self, account: String) -> u64{
    let accounts = Self::get_accounts();
    match accounts.get(& account) {
      Some(&balance) => balance,
      _ => panic!("Don't have {:?} account.", account),
    }
  }

  fn get_bc() -> Blockchain {
    let path = Path::new("ledger.json");
    {
      let contents = read_to_string(path).unwrap();
      serde_json::from_str::<Blockchain>(&contents).unwrap()
    }
  }

  fn get_blocks() -> Vec<Block> {
    Self::get_bc().blocks
  }

  fn get_accounts() -> HashMap<String, u64> {
    Self::get_bc().accounts
  }
}