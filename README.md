# fcf
fcf is a simple rust cli tool built to make accessing your configs faster. You can use it to bind paths to config files with configs' names.

## Installation

### Dependencies
The minimal supported Rust version is 1.80.1
You'll also need to download Cargo.

### Linux/MacOs
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://sh.rustup.rs -sSf | sh
```

### Windows
You can install Cargo on Windows on your own responsibility(however using Windows is not advised and there aren't many uses for fcf there).
These are the official guides for installing rust [rust](https://www.rust-lang.org/tools/install) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Install the fcf crate from crates.io
Just run
```bash
cargo install fcf
```

## Usage
```
fcf <COMMAND>

Commands:
  edit, e            Edit a specific config file
  editor             Set the default editor, you can also do it by setting enviroment variable EDITOR
  bind, b            Bind a key to a file
  remove-binding, r  Remove a binding
  print              Print the current configuration
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
