# Aliasmanager

Aliasmanager is a small CLI utility to quickly add or remove some shell aliases.

## Usage

There are 3 available commands for now:

- `aliasmanager add <alias> <command>`: adds a new alias to `~/.aliasmanagerrc`
- `aliasmanager remove <alias>`: removes the given alias
- `aliasmanager list`: shows all aliases defined in `~/.aliasmanagerrc

## Installation from source

### Pre-requisites

- Have [Rust](https://www.rust-lang.org/learn/get-started) installed

### Steps

1. Clone the repository
2. Navigate to the Aliasmanager app: `cd apps/aliasmanager`
3. Install Aliasmanager system-wide: `cargo install --path .`
4. Add the following lines to your shell configuration file, in order for it to pick-up your custom aliases:
  ```bash
  if [ -f ~/.aliasmanagerrc ]; then
    source ~/.aliasmanagerrc
  fi;
  ```
5. Reload your shell configuration file (usually `source ~/.bashrc` or `source ~/.zshrc`) and voilà!
