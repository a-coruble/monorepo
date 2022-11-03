use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::Path,
};

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

enum AliasmanagerFilePermissions {
    Append,
    Write,
}

fn handle_remove_command(
    alias_to_remove: RemoveCommand,
    alias_file_path: &Path,
) -> Result<(), String> {
    if let Ok(mut alias_file) =
        open_file_with_permissions(alias_file_path, AliasmanagerFilePermissions::Write)
    {
        let alias_file_reader = BufReader::new(alias_file.by_ref());
        let mut content_to_write = String::new();
        let pattern_to_match = format!("alias {}=", alias_to_remove.alias);
        for (index, line) in alias_file_reader.lines().enumerate() {
            if let Ok(line_content) = line {
                println!("{}", line_content);
                if !line_content.starts_with(&pattern_to_match) {
                    if index == 0 {
                        content_to_write = format!("{}", line_content);
                    } else {
                        content_to_write = format!("{}\n{}", content_to_write, line_content);
                    }
                }
            } else {
                panic!("Couldn't read aliases from the alias file (~/.aliasmanagerrc)");
            }
        }
        println!("{}", content_to_write);
        alias_file.set_len(0).unwrap();
        alias_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        alias_file
            .write_all(content_to_write.as_bytes())
            .map_err(|err| {
                format!(
                    "Couldn't write new content after removal, underlying error is: \n{}",
                    err
                )
            })
    } else {
        Err(String::from(
            "Couldn't load aliasmanager file to remove aliases",
        ))
    }
}

fn handle_add_command(alias_to_add: AddCommand, alias_file_path: &Path) -> Result<(), String> {
    if let Ok(mut alias_file) =
        open_file_with_permissions(alias_file_path, AliasmanagerFilePermissions::Append)
    {
        let string_to_append = format!(
            "alias {}=\'{}\'\n",
            alias_to_add.alias, alias_to_add.command
        );
        alias_file
            .write_all(string_to_append.as_bytes())
            .map_err(|err| {
                format!(
                    "Couldn't append new alias to file, underlying error is: \n{}",
                    err
                )
            })
    } else {
        Err(String::from(
            "Couldn't load aliasmanager file to append alias",
        ))
    }
}

fn open_file_with_permissions(
    path: &Path,
    permissions: AliasmanagerFilePermissions,
) -> Result<File, String> {
    let mut opener = OpenOptions::new();
    opener.read(true);
    if !path.exists() {
        opener.create(true);
    }
    match permissions {
        AliasmanagerFilePermissions::Append => opener.append(true),
        AliasmanagerFilePermissions::Write => opener.write(true),
    };
    opener
        .open(path)
        .map_err(|err| format!("Couldn't open file with underlying error: \n{}", err))
}

fn main() {
    let cli = CLI::parse();
    if let Some(user_dirs) = UserDirs::new() {
        let aliasmanager_file_path = Path::new(user_dirs.home_dir()).join("./.aliasmanagerrc");
        let result = match cli.command {
            Commands::Add(alias_to_add) => {
                handle_add_command(alias_to_add, &aliasmanager_file_path)
            }
            Commands::Remove(alias_to_remove) => {
                handle_remove_command(alias_to_remove, &aliasmanager_file_path)
            }
        };
        result
            .map_err(|err| panic!("Something went wrong, here's the log:\n{}", err))
            .unwrap();
    } else {
        panic!("Couldn't determine the user's home directory")
    }
}
