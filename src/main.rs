use std::env;
use std::fs::create_dir_all;
use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Error as SerdeError;
use std::process::Command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about=None, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Edit a specific config file
    #[command(alias = "e")]
    Edit {
        /// The key binded to the config file to edit
        key: String,
    },
    /// Set the default editor
    Editor {
        /// The name of the editor
        editor: String,
    },
    /// Bind a key to a file
    #[command(alias = "b")]
    Bind {
        /// The key to bind
        key: String,
        /// The file to bind to the key
        file: String,
    },
    /// Remove a binding
    #[command(alias = "r")]
    RemoveBinding {
        /// The key to remove the binding from
        key: String,
    },
    /// Print the current configuration
    Print,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    editor: Option<String>,
    bindings: Option<HashMap<String, String>>,
}

impl Config {
    fn new() -> Config {
        Config{
            editor: None,
            bindings: Some(HashMap::new()),
        }
    }
}
const FSF_CONFIG_PATH: &str = "~/.config/fcf";

fn expand_tilde(path: &str) -> String{
    if path.starts_with("~"){
        if let Some(home) = env::home_dir(){
            return path.replacen("~", home.to_str().unwrap(), 1);
        }
    }
    return path.to_string();
}
fn touch(path: &str) -> (){
    let expanded_path = expand_tilde(path);
    let path = Path::new(&expanded_path);

    if let Some(parent) = path.parent(){
        let _ = create_dir_all(parent);
    }
    let _ = OpenOptions::new().create(true).write(true).open(path);

}

fn parse_config() -> Config{
    touch(FSF_CONFIG_PATH);

    let config_file = fs::read_to_string(expand_tilde(FSF_CONFIG_PATH)).expect("Cannot read the config file");
    
    let parse: Result<Config, SerdeError> = serde_json::from_str(&config_file);
    let config: Config = match parse {
        Ok(content) => content,
        Err(ref e) if e.is_eof() => Config::new(),
        Err(e) => {
            panic!("Error occurred while parsing the config file: {e}");
        }
    };
    config

}

fn write_config(config: &Config){
    let file = fs::File::create(expand_tilde(FSF_CONFIG_PATH)).expect("Cannot open the config file");

    if let Err(e) = serde_json::to_writer_pretty(file, &config) {
        println!("An error occurred while writing to the config file: {e}");
        return;
    }

}
fn set_default_editor(editors_name: &str) -> (){
    let mut config: Config = parse_config();

    config.editor = Some(editors_name.to_string());
    write_config(&config);
    println!("Successfully set default editor to {editors_name}");
}

fn bind(name: &str, file_path: &str){
    let mut config: Config = parse_config();
    if config.bindings.is_none() {
        config.bindings = Some(HashMap::new());
    }
    config.bindings
        .as_mut()
        .expect("This shouldn't have happened")
        .insert(name.to_string(), file_path.to_string());
    write_config(&config);
    println!("Successfully binded {name} with {file_path}");
}

fn remove_binding(name: &str){
    let mut config: Config = parse_config();
    if config.bindings.is_none() {
        config.bindings = Some(HashMap::new());
    }
    match config.bindings
        .as_mut()
        .expect("Borrowing bindings HashMap failed")
        .remove_entry(name){
            None => {
                println!("This binding does not exist");
                return;
            },
            _ => (),
    };
    write_config(&config);
    println!("Successfully removed {name} binding");

}

fn edit(name: &str) -> (){
    let config: Config = parse_config();
    let _ = Command::new(config.editor
            .expect("you have not configured your default editor"))
        .arg(config.bindings
            .expect("Failed to retrieve bindings")
            .get(name)
            .expect("The binding to {name} does not exist}"))
        .status();


}

fn print_config() -> (){
    touch(FSF_CONFIG_PATH);
    let config_file = fs::read_to_string(expand_tilde(FSF_CONFIG_PATH)).expect("Cannot read the config file");
    if config_file == "" {
        println!("The config is currently empty");
    }else{
        println!("This is the config \n {config_file}");
    }
}
fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Edit { key } => {
            edit(&key);
        },
        Commands::Editor { editor } => {
            set_default_editor(&editor);
        },
        Commands::Bind { key, file } => {
            bind(&key, &file);
        },
        Commands::RemoveBinding { key } => {
            remove_binding(&key);
        },
        Commands::Print => {
            print_config();
        }

    }
}

