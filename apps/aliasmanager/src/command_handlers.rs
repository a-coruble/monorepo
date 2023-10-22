use std::{
    io::{BufRead, BufReader, Seek, Write},
    path::Path,
};

use crate::{
    cli_parser::{AddCommand, Commands, RemoveCommand},
    config_file_manager::{
        open_file_with_permissions, AliasmanagerFilePermissions, CONFIGURATION_FILE_NAME,
    },
};

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
        Err(String::from(format!(
            "Couldn't open {} file to list aliases",
            alias_file_path.to_str().unwrap_or(CONFIGURATION_FILE_NAME),
        )))
    }
}

fn handle_list_command(alias_file_path: &Path) -> Result<(), String> {
    if let Ok(alias_file) =
        open_file_with_permissions(alias_file_path, AliasmanagerFilePermissions::Read)
    {
        let file_reader = BufReader::new(alias_file);
        for line in file_reader.lines() {
            println!("{}", line.unwrap_or("<empty_line>".to_owned()));
        }
        Ok(())
    } else {
        Err(String::from(format!(
            "Couldn't open {} file to list aliases",
            alias_file_path.to_str().unwrap_or(CONFIGURATION_FILE_NAME),
        )))
    }
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
        Err(String::from(format!(
            "Couldn't open {} file to list aliases",
            alias_file_path.to_str().unwrap_or(CONFIGURATION_FILE_NAME),
        )))
    }
}

pub fn handle_commands(
    command: Commands,
    aliasmanager_config_file_path: &Path,
) -> Result<(), String> {
    match command {
        Commands::Add(alias_to_add) => {
            handle_add_command(alias_to_add, aliasmanager_config_file_path)
        }
        Commands::List => handle_list_command(aliasmanager_config_file_path),
        Commands::Remove(alias_to_remove) => {
            handle_remove_command(alias_to_remove, aliasmanager_config_file_path)
        }
    }
}
