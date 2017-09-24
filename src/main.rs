extern crate regex;

pub mod parser;
pub mod vm;

use parser::*;

fn main() {
    let input = "ADD [cake] A\nCALL :my_vm_is_best".to_string();
    let mut scanner = parser::Scanner::from(input);
    let mut ctr = 0;
    while !scanner.is_eof() && ctr < 20 {
        println!("{:?}", scanner.consume_token());
        ctr += 1;
    }
}
