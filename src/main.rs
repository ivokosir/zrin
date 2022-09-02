mod code;
mod expression;
mod generate_code;
mod resolve_names;

use std::fs::File;
use std::io::BufReader;

use expression::Expression;

fn main() {
    let file = File::open("test_simple_in.json").unwrap();
    let reader = BufReader::new(file);

    let expression: Expression = serde_json::from_reader(reader).unwrap();
    let code = generate_code::generate_code(expression);

    let out = serde_json::to_string_pretty(&code).unwrap();

    println!("{}", out);
}
