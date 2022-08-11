use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "evmm.pest"]
pub struct EVMMParser;

pub fn parse_file(unparsed_file: &str) -> Pair<Rule> {
    EVMMParser::parse(Rule::file, unparsed_file)
        .unwrap_or_else(|_| panic!("Error when parsing file"))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let file = r#"
    PUSH1 0x01 //[0x01]
    push5 0x0102030405 //[0x0102030405 0x01]
    CALLER //[CALLER 0x0102030405 0x01]
    "#;

        let parsed_file = parse_file(file);

        println!("{:?}", parsed_file);
    }

    #[test]
    fn test_parse_push1() {
        let file = r#"
        PUSH1 0x01
    "#;

        let parsed_file = parse_file(file);

        println!("{:?}", parsed_file);
    }
}
