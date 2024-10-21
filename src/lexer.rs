use std::{error::Error, fs::File, path::Path};


pub struct Lexer {
    file: File,

}

pub enum Token {
    LBracket,
    Rbracket,
    LParen,
    RParen,
    IntLiteral(i32),
    StringLiteral(String),
    Semicolon,
    Comma,
}


impl Lexer {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            file: File::open(path)?,
        })
    }

    pub fn next(&mut self) -> Option<Token> {
        None
    }
}