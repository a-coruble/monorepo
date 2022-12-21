use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new alias to the current user aliases
    Add(AddCommand),
    /// Remove an existing alias from the current user aliases
    Remove(RemoveCommand),
    /// List all aliases created using aliasmanager
    List,
}

#[derive(Args, Debug)]
pub struct AddCommand {
    /// The name you want to give to your alias, and how you will call it from the terminal
    pub alias: String,
    /// The underlying command you'd like to execute when triggering this alias
    pub command: String,
}

#[derive(Args, Debug)]
pub struct RemoveCommand {
    /// The name of the alias you want to remove
    pub alias: String,
}

pub fn parse_cli() -> CLI {
    CLI::parse()
}
