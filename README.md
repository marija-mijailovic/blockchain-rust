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
    cargo build
  ```
  
3. Run the project 
- Create account
  ``` 
    cargo run create-account <account> 
  ```
- Create transfer
  ``` 
    cargo run create-transaction <from> <to> <amount> 
  ```
- Check account balance  
  ``` 
    cargo run check-balance <account> 
  ```
