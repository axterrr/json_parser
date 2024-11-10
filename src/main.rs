mod cli;
use std::{fs::read_to_string, path::PathBuf};
use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;
use json_parser::*;
use pest::Parser;

fn main() {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    match action {
        Parse { file } => {
            match parse_json(&file) {
                Ok(res) => {
                    println!("File parsed successfully.");
                    println!("{}", res);
                },
                Err(e) => eprintln!("Error parsing file: \n{}", e),
            }
        },
        Help => {
            println!("Usage:");
            println!("  parse --file <PATH>  Parses the specified JSON file");
            println!("  help                 Displays this help message");
            println!("  credits              Displays credits information");
        },
        Credits => {
            println!("JSON Parser CLI v1.0");
            println!("Created by Hibskyi Vladyslav");
        },
    };
}

fn parse_json(file: &PathBuf) -> Result<String, String> {
    let unparsed_file = read_to_string(file)
        .map_err(|_| format!("Could not read file: {:?}", file))?;
    
    let res = JsonGrammar::parse(Rule::json, &unparsed_file)
        .map_err(|e| format!("Parsing error: \n{}", e))?;
    
    Ok(format!("{}", res))
}
