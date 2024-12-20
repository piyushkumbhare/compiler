#![allow(unused)]

use std::{io::Write, path::Path};

use lexer::{LexToken, Lexer};
mod lexer;

mod parser;

/*
	Goals for first language:
	
	let a = 5;
	let b = 2;

	let c = a + b;

	print(c);
*/


fn main() {
	let input: String = std::fs::read("example.plz").unwrap().iter().map(|&x| char::from(x)).collect();

	let mut lexer = Lexer::new(input);

	for token in lexer {
		println!("{:?}", &token);
	}
}
