# Aliasmanager

Aliasmanager is a small CLI utility to quickly add or remove some shell aliases.

## Usage

There are 2 available commands for now:

- `aliasmanager add <alias> <command>`: adds a new alias to `~/.aliasmanager.sh`
- `aliasmanager remove <alias>`: removes the given alias (NOT IMPLEMENTED FOR NOW!)

## Installation from source

### Pre-requisites
- Have [PNPM](https://pnpm.io) installed
- Have [Rust](https://www.rust-lang.org/learn/get-started) installed

### Steps

1. Clone the repository
2. Navigate to the Aliasmanager app: `cd apps/aliasmanager`
3. Install Aliasmanager system-wide: `cargo install --path .`
4. Add the following lines to your shell configuration file, in order for it to pick-up your custom aliases:
  ```bash
  if [ -f ~/.aliasmanager.sh ]; then
    source ~/.aliasmanager.sh
  fi;
  ```
5. Reload your shell configuration file (usually `source ~/.bashrc` or `source ~/.zshrc`) and voilà!
