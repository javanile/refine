
//extern crate lalrpop;

use std::io::{self, Write};
//use refine::define::ExprParser;

mod define;

fn main() {

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            break;
        }

        match ExprParser::new().parse(&input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
