# Blockchain rust
Simple blockchain implementation in Rust

### Prerequisites
  
  Be sure to have installed [Rust](https://www.rust-lang.org/learn/get-started)

### Installation

1. Clone the repo
  ```
    git clone https://github.com/marija-mijailovic/blockchain-rust.git
  ```

2. Build the project
  ```
    cargo install --path . 
  ```
  
3. Run the project 
- Create account
  ``` 
    blockchain_rust create-account <account> 
  ```
- Create transfer
  ``` 
    blockchain_rust create-transaction <from> <to> <amount> 
  ```
- Check account balance  
  ``` 
    blockchain_rust check-balance <account> 
  ```
