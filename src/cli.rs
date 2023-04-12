use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct BlockchainArgs {
  #[clap(subcommand)]
  pub command_type : CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
  /// Create account for transactions
  CreateAccount(CreateAccountCommand),
  /// Create transactions
  CreateTransaction(CreateTransactionCommand),
  /// Get account balance
  CheckBalance(CheckBalanceCommand)
}

#[derive(Debug, Args)]
pub struct CreateAccountCommand {
  pub account: String 
}

#[derive(Debug, Args)]
pub struct CreateTransactionCommand {
  pub from: String,
  pub to: String,
  pub amount: u64
}

#[derive(Debug, Args)]
pub struct CheckBalanceCommand {
  pub account: String 
}