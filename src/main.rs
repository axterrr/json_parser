mod cli;
use cli::{Action::*, CommandLineArgs};
use json_parser::*;
use pest::Parser;
use std::{fs::read_to_string, path::PathBuf};
use structopt::StructOpt;

fn main() {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    match action {
        Parse { file } => match parse_json(&file) {
            Ok(res) => {
                println!("File parsed successfully.");
                println!("{}", res);
            }
            Err(e) => eprintln!("{}", e),
        },
        Help => {
            println!("Usage:");
            println!("  parse --file <PATH>  Parses the specified JSON file");
            println!("  help                 Displays this help message");
            println!("  credits              Displays credits information");
        }
        Credits => {
            println!("JSON Parser CLI v1.0");
            println!("Created by Hibskyi Vladyslav");
        }
    };
}

fn parse_json(file: &PathBuf) -> Result<String, String> {
    let unparsed_file = read_to_string(file)
        .map_err(|_| format!("Failed to read file: {:?}", file))?;
    
    let res = JsonGrammar::parse(Rule::json, &unparsed_file)
        .map_err(|e| format!("Syntax error while parsing JSON: \n{}", e))?;
    
    Ok(format!("{}", res))
}
