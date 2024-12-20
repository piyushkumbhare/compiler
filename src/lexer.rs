use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

use regex::Regex;

pub struct Lexer {
    text: String,
    cursor: usize,
    re: Regex,
    capture_names: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
}

#[derive(Debug, PartialEq)]
pub enum LexToken {
    Id(String),
    Num(i32),

    Assign,
    Add,
    Sub,
    Mult,
    Div,

    Whitespace,
    Semicolon,

    Keyword(Keyword),

    EOF,
    
    LPar,
    RPar,
}

impl Lexer {
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        Ok(Self::new(
            fs::read(path)?.iter().map(|&c| char::from(c)).collect(),
        ))
    }

    pub fn new(input: String) -> Self {
        let re = r#"(?P<ID>^[a-z]+[0-9]*)|(?P<NUM>^[0-9]+)|(?P<ASSIGN>^=)|(?P<ADD>^\+)|(?P<SUB>^-)|(?P<MULT>^\*)|(?P<DIV>^/)|(?P<IGNORE>^\s)|(?P<SEMICOLON>^;)|(?P<LPAR>\()|(?P<RPAR>\))"#;
        let re = Regex::new(&re).expect("Error compiling Regex");

        let capture_names: Vec<String> = re
            .capture_names()
            .filter_map(|x| match x {
                Some(s) => Some(s.to_string()),
                None => None,
            })
            .collect();

        Self {
            text: input,
            cursor: 0,
            re,
            capture_names,
        }
    }
}

impl<'a> Iterator for Lexer {
    type Item = LexToken;

    /// Returns the next token as a `Some(LexToken)`, or `None` if `EOF` is reached
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.text.len() {
            // Run the regex on the remaining slice
            let captures = self
                .re
                .captures(&self.text[self.cursor..])
                .expect("Lexing Error: Unknown tokens seen");

            // Find out which name was matched & captured
            for name in self.capture_names.iter() {
                let capture = captures.name(name);
                if capture.is_some() {
                    let value = captures.name(name).unwrap().as_str().to_string();
                    self.cursor += value.len();
                    let token = match name.as_str() {
                        "ID" => LexToken::Id(value),
                        "NUM" => LexToken::Num(
                            value.trim().parse().expect("Error parsing integer literal"),
                        ),

                        "ASSIGN" => LexToken::Assign,
                        "ADD" => LexToken::Add,
                        "SUB" => LexToken::Sub,
                        "MULT" => LexToken::Mult,
                        "DIV" => LexToken::Div,

                        "LPAR" => LexToken::LPar,
                        "RPAR" => LexToken::RPar,


                        // If we see a whitespace, ignore it and return the next Some(token)
                        "IGNORE" => return self.next(),
                        "SEMICOLON" => LexToken::Semicolon,
                        x => LexToken::EOF,
                    };
                    return Some(token);
                }
            }
        }
        None
    }
}
