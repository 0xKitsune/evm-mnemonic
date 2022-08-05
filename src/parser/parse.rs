use pest::Parser;

#[derive(Parser)]
#[grammar = "evmm.pest"]
pub struct EVMMParser;

fn parse_file(unparsed_file: String) {
    let file = EVMMParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
}

#[test]
fn test_parse_file() {}
