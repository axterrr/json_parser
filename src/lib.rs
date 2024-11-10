use pest_derive::Parser;
use thiserror::Error;
use pest::error::Error as PestError;

#[derive(Parser)]
#[grammar = "./json_grammar.pest"]
pub struct JsonGrammar;

#[derive(Error, Debug)]
pub enum JsonParserError {
    #[error("Syntax error while parsing JSON: \n{0}")]
    Syntax(#[from] PestError<Rule>),

    #[error("Failed to read file: \n{0}")]
    FileReadError(#[from] std::io::Error),
}
