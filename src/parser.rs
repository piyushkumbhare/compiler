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

/*
	Simplified Grammar:

    <program>			::=		<statement_list>

    <statement_list>	::=		<statement> <statement_list>
                                | eps

    <statement>			::= 	"let" <variable> "=" <expression> ";"
                                | <variable> "=" <expression> ";"
                                "print" <expression> ";"

    <expression>        ::=     <term> <expression'>								

	<expression'>		::=		"+" <expression>
								| "-" <expression>
								| eps

	<term>				::=		"(" <expression> ")" <term'>
								| VAR <term'>
								| NUM <term'>

	<term'>				::=		"*" <term>
								| "/" <term>
								| eps

	*/

use std::{collections::HashMap, iter::Peekable};

use crate::lexer::Lexer;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

pub struct ParseError;

pub struct Program<'a> {
    statement_list: Vec<Statement<'a>>,
}

pub enum Statement<'a> {
	Let(String, Expression<'a>),
	Assign(String, Expression<'a>),
	Print(Term<'a>),
}

pub enum Expression<'a> {
	Add(&'a Expression<'a>, &'a Expression<'a>),
	Sub(&'a Expression<'a>, &'a Expression<'a>),
	Term(Term<'a>)
}

pub enum Term<'a> {
	Mult(&'a Term<'a>, &'a Term<'a>),
	Div(&'a Term<'a>, &'a Term<'a>),
	Parenthesis(&'a Expression<'a>),
	Num(i32),
}

impl Parser {
	pub fn new(input: String) -> Self {
		Self {
			lexer: Lexer::new(input).peekable(),
		}
	}

	/// Parses the input and returns a `Program` struct representing its AST
    pub fn parse<'a>(mut self) -> Result<Program<'a>, ParseError> {
		todo!()
	}

	fn parse_statement<'a>(&mut self) -> Result<Statement<'a>, ParseError> {
		todo!()
	}

	fn parse_expression<'a>(&mut self) -> Result<Expression<'a>, ParseError> {
		todo!()
	}

	fn parse_term<'a>(&mut self) -> Result<Term<'a>, ParseError> {
		let Some(token) = self.lexer.next() else {
			return Err(ParseError);
		};

		use crate::LexToken::*;
		match token {
			Id(s) => todo!(),
			Num(i) => todo!(),
			LPar => {
				let expr = self.parse_expression()?;
				let Some(next) = self.lexer.next() else {
					return Err(ParseError);
				};
				if next != RPar {
					return Err(ParseError);
				}
				Ok(Term::Parenthesis(expr))
			},
			_ => Err(ParseError),
		}
	}
}
