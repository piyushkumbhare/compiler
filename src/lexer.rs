use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

use regex::Regex;

pub struct Lexer {
    text: String,
    cursor: usize,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
}

#[derive(Debug)]
pub enum LexToken<'a> {
    Id(&'a str),
    Num(i32),

    Assign,
    Add,
    Sub,
    Mult,
    Div,

    Whitespace,
    Semicolon,

    Keyword(Keyword),
}

impl<'a> Lexer {
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        Ok(Self::new(
            fs::read(path)?.iter().map(|&c| char::from(c)).collect(),
        ))
    }

    pub fn new(input: String) -> Self {
        Self {
            text: input,
            cursor: 0,
        }
    }

    pub fn lex(&'a mut self) -> Vec<LexToken<'a>> {
        let mut tokens = Vec::new();

        let re = r#"(?P<ID>[a-z]+)|(?P<NUM>[0-9]+)|(?P<ASSIGN>=)|(?P<ADD>\+)|(?P<SUB>-)|(?P<MULT>\*)|(?P<DIV>/)|(?P<IGNORE> |\n)|(?P<SEMICOLON>\;)"#;
        let re = Regex::new(&re).expect("Error compiling Regex");

        let capture_names: Vec<_> = re.capture_names().filter_map(|x| x).collect();
        println!("{:?}", capture_names);

        while self.cursor < self.text.len() {
            let captures = re
                .captures(&self.text[self.cursor..])
                .expect("Lexing Error: Unknown tokens seen");

            let Some(token) = (1..=9).find_map(|n| captures.get(n)) else {
                break;
            };

            for &name in capture_names.iter() {
                let capture = captures.name(name);
                if capture.is_some() {
                    let value = captures.name(name).unwrap().as_str();
                    self.cursor += value.len();
                    let token = match name {
                        "ID" => LexToken::Id(value),
                        "NUM" => LexToken::Num(value.parse().expect("Error parsing integer literal")),
                        
                        "ASSIGN" => LexToken::Assign,
                        "ADD" => LexToken::Add,
                        "SUB" => LexToken::Sub,
                        "MULT" => LexToken::Mult,
                        "DIV" => LexToken::Div,
                        
                        "IGNORE" => break,
                        "SEMICOLON" => LexToken::Semicolon,
                        x => panic!("Unexpected token parsed: `{x}`")
                    };
                    println!("{:?}", token);
                    tokens.push(token);
                    break;
                }
            }
        }
        tokens
    }
}
