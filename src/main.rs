#[macro_use]
extern crate lalrpop_util;

mod tokens;

lalrpop_mod!(pub nomnoml);

fn main() {
    // assert!(calculator1::TermParser::new().parse("22").is_ok());
    // assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    // assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    // assert!(calculator1::TermParser::new().parse("((22)").is_err());
}
