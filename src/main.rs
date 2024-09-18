use std::env;
use std::fs::create_dir_all;
use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Error as SerdeError;
use std::process::Command;

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
const INFO: &str = "
Usage:

    --help, -h, help
        Prints help(the same thing you are seeing right now)
    bind, b <name> <config_file_path>
        Binds a name with configs file path.
    remove_binding, r <name>
        Removes a binding with provided name
    editor <editors_name>
        Configures the default editor that will be used for configuring
    edit <name>
        Edit a config with the default editor
    print
        Print the current state of the config
    ";
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
fn help() -> (){
    println!("{}", INFO);
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

fn add_change_binding(name: &str, file_path: &str){
    let mut config: Config = parse_config();
    if config.bindings.is_none() {
        config.bindings = Some(HashMap::new());
    }
    config.bindings
        .as_mut()
        .expect("This shouldn't have happened")
        .entry(name.to_string())
        .and_modify(|counter| *counter = file_path.to_string())
        .or_insert(file_path.to_string());
    write_config(&config);

}

fn remove_binding(name: &str){
    let mut config: Config = parse_config();
    if config.bindings.is_none() {
        config.bindings = Some(HashMap::new());
    }
    match config.bindings
        .as_mut()
        .expect("This shouldn't have happened")
        .remove_entry(name){
            None => {
                println!("This binding does not exist");
                return;
            },
            _ => (),
    };
    write_config(&config);


}

fn edit(name: &str) -> (){
    let config: Config = parse_config();
    let _ = Command::new(config.editor
            .expect("you have not configured your default editor"))
        .arg(config.bindings
            .expect("This shouldn't have happened")
            .get(name)
            .expect("The binding to {name} does not exist}"))
        .status();


}

fn print_config() -> (){
    touch(FSF_CONFIG_PATH);
    let config_file = fs::read_to_string(expand_tilde(FSF_CONFIG_PATH)).expect("Cannot read the config file");
    if config_file == "" {
        println!("the config is currently empty");
    }else{
        println!("this is the config \n {config_file}");
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // empty args
    if args.len() == 1{
        help();
    } else if args.len() >= 3 && args[1] == "edit" {
        edit(&args[2]);
    } else if args.len() >= 3 && args[1] == "editor" {
        set_default_editor(&args[2]);
    }else if args.len() >= 4 && (args[1] == "bind" || args[1] == "b") {
        add_change_binding(&args[2],&args[3]);   
    } else if args.len() >= 3 && (args[1] == "remove_binding" || args[1] =="r") {
        remove_binding(&args[2]);
    } else if args.len() >=2 && args[1] == "print" {
        print_config();
    } else if args.len() >=2 && (args[1] == "--help" || args[1] == "-h" || args[1] == "help"){
        help();
    }
}

