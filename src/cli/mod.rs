pub mod commands;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// starts guilder server
  Start,
  /// create new user
  Register {
    #[clap(short, long)]
    username: String,
    #[clap(short, long)]
    password: String,
  }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct GuilderCli {
  #[clap(subcommand)]
  pub action: Option<Commands>,
}
