use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Token {
	EndOfFile,
	None,
	Literal(Literal),
	Plus,
	Minus,
	Asterisk,
	Slash,
	Semicolon,
	Equals,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Token::EndOfFile => write!(f, "EOF"),
			Token::None => write!(f, "None"),
			Token::Literal(_) => write!(f, "Literal"),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Asterisk => write!(f, "*"),
			Token::Slash => write!(f, "/"),
			Token::Semicolon => write!(f, ";"),
			Token::Equals => write!(f, "="),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
	Let,
	Print,
	Symbol(String),
}

impl fmt::Display for Identifier {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Identifier::Print => write!(f, "print"),
			Identifier::Let => write!(f, "let"),
			Identifier::Symbol(s) => write!(f, "{s}"),
		}
	}
}

pub const IDENTIFIER_SYMBOLS: &[(&str, Identifier)] = &[
	("let", Identifier::Let),
	("print", Identifier::Print),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
	Integer(i64),
	Identifier(Identifier)
}

pub const TOKEN_SYMBOLS: &[(&str, Token)] = &[
	("+", Token::Plus),
	("-", Token::Minus),
	("*", Token::Asterisk),
	("/", Token::Slash),
	(";", Token::Semicolon),
	("=", Token::Equals),
];