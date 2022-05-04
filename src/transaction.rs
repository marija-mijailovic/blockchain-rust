use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Transaction {
  pub from: String,
  pub to: String,
  pub value: u32,
  pub timestamp: u128
}

impl Transaction {
  pub fn new(
    from: String,
    to: String,
    value: u32
  ) -> Transaction {
    Transaction {
      from,
      to,
      value,
      timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
  }

  pub fn bytes(&self) -> Vec<u8> {
    let mut bytes = vec![];

    bytes.extend(self.from.as_bytes());
    bytes.extend(self.to.as_bytes());
    bytes.extend(&self.value.to_ne_bytes());
    bytes.extend(&self.timestamp.to_ne_bytes());

    bytes
  }
}