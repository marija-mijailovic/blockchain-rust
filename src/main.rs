use blockchain_rust::{cli::{BlockchainArgs, CommandType}, blockchain::Blockchain, transaction::Transaction};
use std::fs::write;
use clap::Parser;

fn main() {
    let subcommand = BlockchainArgs::parse().command_type;

    let mut blockchain = Blockchain::get();

    match subcommand {
        CommandType::CreateAccount(args) => {
            blockchain.add_accounts(args.account.to_owned());
        },
        CommandType::CreateTransaction(args) => {
            let tx = Transaction::new(
                args.from.to_owned(), 
                args.to.to_owned(), 
                args.amount.to_owned());
            blockchain.add_transaction(tx);
        },
        CommandType::CheckBalance(args) => {
            let balance = blockchain.account_balance(args.account.to_owned());
            println!("Balance of {:?} : {:?}", args.account, balance);
        }
    };

    write(
        "ledger.json", 
        serde_json::to_string_pretty(&blockchain).unwrap())
    .expect("Problem writing to ledger.json");
}
