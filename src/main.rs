#![allow(unused)]

use std::path::Path;

use lexer::Lexer;
mod lexer;

/*
    Goals for first language:
    
    let a = 5;
    let b = 2;

    let c = a + b;

    print(c);
*/

fn main() {
    let mut lexer = Lexer::from_path(Path::new("example.plz")).unwrap();

    let tokens = lexer.lex();
    println!("{:?}", &tokens);
}
