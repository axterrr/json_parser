use pest::error::Error as PestError;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./json_grammar.pest"]
pub struct JsonGrammar;

#[derive(Error, Debug)]
#[error("Syntax error while parsing JSON: \n{0}")]
pub struct JsonParserError(#[from] PestError<Rule>);
