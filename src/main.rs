use std::env;
use std::fs::create_dir_all;
use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Error as SerdeError;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    editor: Option<String>,
    bindings: Option<HashMap<String, String>>,
}

impl Config {
    fn new() -> Config {
        Config{
            editor: None,
            bindings: None,
        }
    }
}
const INFO: &str = "
Usage:

    bind, bnd <name> <config_file_path>
        Binds a name with configs file path.

    editor <editors_name>
        Configures the default editor that will be used for configuring

    edit <name>
        Edit a config with the default editor
    ";
const FSF_CONFIG_PATH: &str = "~/.config/fsf";
fn expand_tilde(path: &str) -> String{
    if path.starts_with("~"){
        if let Some(home) = env::home_dir(){
            return path.replacen("~", home.to_str().unwrap(), 1);
        }
    }
    return path.to_string();
}
fn touch(path: &str){
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

fn set_default_editor(editors_name: &str){
    touch(FSF_CONFIG_PATH);
    let config_file: &str = &fs::read_to_string(expand_tilde(FSF_CONFIG_PATH)).expect("Cannot read the config file");
    let parse: Result<Config, SerdeError> = serde_json::from_str(&config_file);
    let config: Config = match parse {
        Ok(content) => content,
        Err(ref e) if e.is_eof() => Config::new(),
        Err(e) => panic!("{e}"),
    };


    println!("Successfully set default editor to {editors_name}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // empty args
    if args.len() == 1{
        help();
    } else if args.len() == 2 && args[1] == "edit"{
        println!("edit");
    } else if args.len() == 3 && args[1] == "editor"{
        set_default_editor(&args[2]);
    }
}

