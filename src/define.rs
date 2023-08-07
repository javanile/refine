// calc.rs
use super::ExprParser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub define);

pub fn parse(input: &str) -> Result<f64, String> {
    let parser = ExprParser::new();
    match parser.parse(input) {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("{:?}", e)),
    }
}
