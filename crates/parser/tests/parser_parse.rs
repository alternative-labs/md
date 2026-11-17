use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use md_parser::parser::Parser;

#[test]
fn parse_text_fixture() {
    let fixture_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/text.md");
    let source: String = read_to_string(fixture_path).unwrap();
    let mut parser: Parser<'_> = Parser::from_string(&source);

    parser.parse();

    println!();
}
