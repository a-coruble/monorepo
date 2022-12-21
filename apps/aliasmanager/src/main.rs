use directories::UserDirs;
use std::path::Path;

pub mod cli_parser;
pub mod command_handlers;
pub mod config_file_manager;
use crate::{
    cli_parser::parse_cli, command_handlers::handle_commands,
    config_file_manager::CONFIGURATION_FILE_NAME,
};

fn main() {
    let cli = parse_cli();
    if let Some(user_dirs) = UserDirs::new() {
        let aliasmanager_config_file_path =
            Path::new(user_dirs.home_dir()).join(CONFIGURATION_FILE_NAME);
        handle_commands(cli.command, &aliasmanager_config_file_path)
            .map_err(|err| panic!("Something went wrong, here's the log:\n{}", err))
            .unwrap();
    } else {
        panic!("Couldn't determine the user's home directory")
    }
}
