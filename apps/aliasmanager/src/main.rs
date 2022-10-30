use std::{fs::File, io::Write, path::Path};

use clap::{Args, Parser, Subcommand};
use directories::UserDirs;

#[derive(Debug, Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new alias to the current user aliases
    Add(AddCommand),
    /// Remove an existing alias from the current user aliases
    Remove(RemoveCommand),
}

#[derive(Args, Debug)]
struct AddCommand {
    /// The name you want to give to your alias, and how you will call it from the terminal
    alias: String,
    /// The underlying command you'd like to execute when triggering this alias
    command: String,
}

#[derive(Args, Debug)]
struct RemoveCommand {
    /// The name of the alias you want to remove
    alias: String,
}

fn handle_add_command(alias_to_add: AddCommand, alias_file: &mut File) {
    let string_to_append = format!("alias {}=\'{}\'\n", alias_to_add.alias, alias_to_add.command);
    alias_file.write_all(string_to_append.as_bytes()).unwrap();
}

fn get_or_create_aliasmanager_file() -> Result<File, std::io::Error> {
    if let Some(user_dirs) = UserDirs::new() {
        let alias_file_path = Path::new(user_dirs.home_dir()).join("./.aliasmanager.sh");

        if Path::exists(alias_file_path.as_path()) {
            File::options()
                .append(true)
                .read(true)
                .open(alias_file_path)
        } else {
            File::options()
                .append(true)
                .create(true)
                .read(true)
                .open(alias_file_path)
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Couldn't determine the user's home directory",
        ))
    }
}

fn main() {
    let cli = CLI::parse();
    if let Ok(mut alias_file) = get_or_create_aliasmanager_file() {
        match cli.command {
            Commands::Add(alias_to_add) => handle_add_command(alias_to_add, &mut alias_file),
            Commands::Remove(alias_to_remove) => {
                println!("Alias to remove: {}", alias_to_remove.alias)
            }
        }
    } else {
        panic!("Something went wrong when trying to load / create Aliasmanager files, here's the underlying error \n")
    }
}
