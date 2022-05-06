use clap::{arg, Command};
use std::fs::write;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub struct Cli {
  pub blockchain: Blockchain
}

impl Cli {
  pub fn new () -> Self{
    Cli {
      blockchain: Blockchain::get()
    }
  }
  pub fn run(&mut self) {
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
      subcommand(
        Command::new("check-balance").
            about("Get account balance").
            arg(arg!(<account> "Account to check balance for")).
            arg_required_else_help(true)).
      get_matches();

    if let Some(matches) = matches.subcommand_matches("create-account") {
        if let Some(account) = matches.value_of("account") {
            self.blockchain.add_accounts(account.to_owned());
        } else {
            println!("Invalid account command");
        }
    } else if let Some(matches) = matches.subcommand_matches("create-transaction") {
        if let Some(from) = matches.value_of("from") {
            if let Some(to) = matches.value_of("to") {
                if let Some(amount) = matches.value_of("amount") {
                    let tx1 = Transaction::new(from.to_owned(), to.to_owned(), amount.to_owned().parse().unwrap());
                    self.blockchain.add_transaction(tx1);
                } else {
                    println!("Invalid amount command");
                }
            } else {
                println!("Invalid to command");
            }
        } else {
            println!("Invalid from command");
        }
    } else if let Some(matches) = matches.subcommand_matches("check-balance") {
      if let Some(account) = matches.value_of("account") {
        let balance = self.blockchain.account_balance(account.to_owned());
        println!("Balance of {:?} : {:?}",account, balance);
      } else {
          println!("Invalid account command");
      }
    }
    write("ledger.json", serde_json::to_string_pretty(&self.blockchain).unwrap()).expect("Problem writing");
  }
}