use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./json_grammar.pest"]
pub struct JsonGrammar;
