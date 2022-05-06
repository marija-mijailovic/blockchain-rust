use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
  pub id: u32,
  pub timestamp: u128,
  pub hash: Vec<u8>,
  pub prev_block_hash: Vec<u8>,
  pub nonce: u64,
  pub transactions: Vec<Transaction>
}

impl Block {
  pub fn new(
    transactions: Vec<Transaction>,
    prev_block: &Block,
  ) -> Self {
    Block{
      id: prev_block.id,
      timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
      hash: vec![0; 32],
      prev_block_hash: prev_block.hash.clone(), 
      nonce: 0,
      transactions: transactions
    }
  }

  pub fn genesis() -> Self {
    Block {
      id: 0,
      timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
      hash: vec![0; 32],
      prev_block_hash: vec![0; 32],
      nonce: 0,
      transactions: vec![]
    }
  }

  pub fn bytes(&self) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend(&self.id.to_ne_bytes());
    bytes.extend(&self.timestamp.to_ne_bytes());
    bytes.extend(&self.prev_block_hash);
    bytes.extend(&self.nonce.to_ne_bytes());
    bytes.extend(self.transactions.
      iter().
      flat_map(|transaction| transaction.bytes()).
      collect::<Vec<u8>>()
    );

    bytes
  }

  pub fn set_hash(&mut self) {
    self.hash = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
  }
}
