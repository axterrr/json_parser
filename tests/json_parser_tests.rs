use json_parser::*;
use pest::Parser;

#[cfg(test)]
mod json_parser_tests {
    use super::*;

    #[test]
    fn test_parse_rule_json() -> anyhow::Result<()> {
        let data_to_parse1 = "null";
        let data_to_parse2 = "true";
        let data_to_parse3 = "-23.5";
        let data_to_parse4 = "\"string\"";
        let data_to_parse5 = "[1,2,3]";
        let data_to_parse6 = "{\"aaa\":123}";

        let parsed_data1 = JsonGrammar::parse(Rule::json, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::json, data_to_parse2)?;
        let parsed_data3 = JsonGrammar::parse(Rule::json, data_to_parse3)?;
        let parsed_data4 = JsonGrammar::parse(Rule::json, data_to_parse4)?;
        let parsed_data5 = JsonGrammar::parse(Rule::json, data_to_parse5)?;
        let parsed_data6 = JsonGrammar::parse(Rule::json, data_to_parse6)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);
        assert_eq!(parsed_data4.as_str(), data_to_parse4);
        assert_eq!(parsed_data5.as_str(), data_to_parse5);
        assert_eq!(parsed_data6.as_str(), data_to_parse6);

        Ok(())
    }

    #[test]
    fn test_parse_rule_value() -> anyhow::Result<()> {
        let data_to_parse1 = "null";
        let data_to_parse2 = "true";
        let data_to_parse3 = "-23.5";
        let data_to_parse4 = "\"string\"";
        let data_to_parse5 = "[1,2,3]";
        let data_to_parse6 = "{\"aaa\":123}";

        let parsed_data1 = JsonGrammar::parse(Rule::value, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::value, data_to_parse2)?;
        let parsed_data3 = JsonGrammar::parse(Rule::value, data_to_parse3)?;
        let parsed_data4 = JsonGrammar::parse(Rule::value, data_to_parse4)?;
        let parsed_data5 = JsonGrammar::parse(Rule::value, data_to_parse5)?;
        let parsed_data6 = JsonGrammar::parse(Rule::value, data_to_parse6)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);
        assert_eq!(parsed_data4.as_str(), data_to_parse4);
        assert_eq!(parsed_data5.as_str(), data_to_parse5);
        assert_eq!(parsed_data6.as_str(), data_to_parse6);

        Ok(())
    }

    #[test]
    fn test_parse_rule_null() -> anyhow::Result<()> {
        let data_to_parse = "null";

        let parsed_data = JsonGrammar::parse(Rule::null, data_to_parse)?;

        assert_eq!(parsed_data.as_str(), data_to_parse);

        Ok(())
    }

    #[test]
    fn test_parse_rule_boolean() -> anyhow::Result<()> {
        let data_to_parse1 = "true";
        let data_to_parse2 = "false";

        let parsed_data1 = JsonGrammar::parse(Rule::boolean, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::boolean, data_to_parse2)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);

        Ok(())
    }

    #[test]
    fn test_parse_rule_number() -> anyhow::Result<()> {
        let data_to_parse1 = "-560";
        let data_to_parse2 = "560";
        let data_to_parse3 = "0";
        let data_to_parse4 = "25.25";

        let parsed_data1 = JsonGrammar::parse(Rule::number, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::number, data_to_parse2)?;
        let parsed_data3 = JsonGrammar::parse(Rule::number, data_to_parse3)?;
        let parsed_data4 = JsonGrammar::parse(Rule::number, data_to_parse4)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);
        assert_eq!(parsed_data4.as_str(), data_to_parse4);

        Ok(())
    }

    #[test]
    fn test_parse_rule_string() -> anyhow::Result<()> {
        let data_to_parse = "\"   Just a string   \"";

        let parsed_data = JsonGrammar::parse(Rule::string, data_to_parse)?;

        assert_eq!(parsed_data.as_str(), data_to_parse);

        Ok(())
    }

    #[test]
    fn test_parse_rule_array() -> anyhow::Result<()> {
        let data_to_parse1 = "[]";
        let data_to_parse2 = "[45]";
        let data_to_parse3 = 
            "[
                \"bla-bla\",
                345.65,
                null,
                true,
                [],
                {}
            ]";

        let parsed_data1 = JsonGrammar::parse(Rule::array, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::array, data_to_parse2)?;
        let parsed_data3 = JsonGrammar::parse(Rule::array, data_to_parse3)?;
        
        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);

        Ok(())
    }

    #[test]
    fn test_parse_rule_field() -> anyhow::Result<()> {
        let data_to_parse1 = "\"key\":125";
        let data_to_parse2 = "\"key\":\"125\"";
        let data_to_parse3 = "\"key\":[1,5.6,null]";

        let parsed_data1 = JsonGrammar::parse(Rule::field, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::field, data_to_parse2)?;
        let parsed_data3 = JsonGrammar::parse(Rule::field, data_to_parse3)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);

        Ok(())
    }

    #[test]
    fn test_parse_rule_object() -> anyhow::Result<()> {
        let data_to_parse1 = "{}";
        let data_to_parse2 = "{\"key\"  :  \"value\"}";
        let data_to_parse3 = 
            "{
                \"name\": \"Artur\", 
                \"age\": -0.67, 
                \"occupation\": \"med_worker\", 
                \"has_chronical_illness\": true
            }";

        let parsed_data1 = JsonGrammar::parse(Rule::object, data_to_parse1)?;
        let parsed_data2 = JsonGrammar::parse(Rule::object, data_to_parse2)?;
        let parsed_data3 = JsonGrammar::parse(Rule::object, data_to_parse3)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);

        Ok(())
    }
}
