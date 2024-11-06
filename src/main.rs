use json_parser::*;
use pest::Parser;

fn main() -> anyhow::Result<()>{

    let json = 
        "{
            \"name\": \"Artur\", 
            \"age\": -0.67, 
            \"occupation\": \"med_worker\", 
            \"has_chronical_illness\": true
        }";
    let got1 = JsonGrammar::parse(Rule::json, json)?;
    print!("{:?}", got1);

    let field = "\"name\":\"Artur\"";
    let got2 = JsonGrammar::parse(Rule::field, field)?;
    print!("{:?}", got2);

    let number = "-245.243";
    let got3 = JsonGrammar::parse(Rule::number, number)?;
    print!("{:?}", got3);

    Ok(())
}
