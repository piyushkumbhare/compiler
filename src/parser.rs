/*
    Language Grammar:

    <program>			::=		<statement_list>

    <statement_list>	::=		<statement> <statement_list>
                                | eps

    <statement>			::= 	"let" <variable> = <expression>;
                                | <variable> = <expression>;
                                "print" <expression>;

    <expression>        ::=     <expression> "+" <expression>
                                | <expression> "-" <expression>
                                | <term>

    <term>				::=		<term> "*" <term>
                                | <term> "/" <term>
                                | "(" <expression> ")"
                                | VAR
                                | NUM
*/

// Simplified Grammar:
//
// <program>			::=		<statement_list>
//
// <statement_list>	::=		<statement> <statement_list>
//                             | eps
//
// <statement>			::= 	"let" <variable> "=" <expression> ";"
//                             | <variable> "=" <expression> ";"
//                             "print" <expression> ";"
//
// <expression>        ::=     <term> <expression'>
//
// <expression'>		::=		"+" <expression>
//                             | "-" <expression>
//                             | eps
//
// <term>				::=		"(" <expression> ")" <term'>
//                             | VAR <term'>
//                             | NUM <term'>
//
// <term'>				::=		"*" <term>
//                             | "/" <term>
//                             | eps

use std::{collections::HashMap, iter::Peekable};

use crate::lexer::{LexToken, Lexer};

pub struct Parser {
    lexer: Peekable<Lexer>,
}

pub struct ParseError;

pub struct Program {
    statement_list: Vec<Statement>,
}

pub enum Statement {
    Let(String, Box<Expression>),
    Assign(String, Expression),
    Print(Term),
}

pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Term(Term),
}

pub enum Term {
    Mult(Box<Term>, Box<Term>),
    Div(Box<Term>, Box<Term>),
    Parenthesis(Box<Expression>),
    Num(i32),
    Id(String),
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    /// Parses the input and returns a `Program` struct representing its AST
    pub fn parse(mut self) -> Result<Program, ParseError> {
        todo!()
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        todo!()
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        todo!()
    }

    fn parse_expression_prime(&mut self) -> Result<Expression, ParseError> {
        todo!()
    }

    // <term>				::=		"(" <expression> ")" <term'>
    //                             | VAR <term'>
    //                             | NUM <term'>
    // First: LPAR | VAR | NUM
    // Follow: SEMI | ADD | SUB | RPAR
    fn parse_term(&mut self) -> Result<Term, ParseError> {
        let Some(token) = self.lexer.next() else {
            return Err(ParseError);
        };

        let token = match token {
            LexToken::Id(var) => Ok(Term::Id(var)),
            LexToken::Num(i) => Ok(Term::Num(i)),
            LexToken::LPar => {
                let expr = self.parse_expression()?;
                let Some(next) = self.lexer.next() else {
                    return Err(ParseError);
                };
                if next != LexToken::RPar {
                    return Err(ParseError);
                }
                Ok(Term::Parenthesis(Box::new(expr)))
            }
            _ => Err(ParseError),
        };

        let next = self.parse_term_prime()?;
        todo!()
    }

    // <term'>				::=		"*" <term>
    //                             | "/" <term>
    //                             | eps
    // First: MUL | DIV
    // Follow: SEMI | ADD | SUB | RPAR
    fn parse_term_prime(&mut self) -> Result<Option<(LexToken, Term)>, ParseError> {
        let Some(peek) = self.lexer.peek() else {
            return Err(ParseError);
        };

        // Peek token in case epsilon path is taken
        match peek {
            LexToken::Op(op) => {
				self.lexer.next();
                match op.as_str() {
					"*" => Ok(Some(()))
				}
                Ok(Some((op, self.parse_term()?)))
            }
            _ => {
                let Some(peek) = self.lexer.peek() else {
                    return Err(ParseError);
                };
                match peek {
                    LexToken::Semicolon | LexToken::Add | LexToken::Sub | LexToken::RPar => {
                        Ok(None)
                    }
                    _ => return Err(ParseError),
                }
            }
        }
    }
}
