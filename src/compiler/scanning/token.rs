use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Token {
	EndOfFile,
	None,
	Literal(Literal),
	Plus,
	Minus,
	Asterisk,
	Slash,
	Semicolon,
	Identifier(Identifier),
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
			Token::Identifier(_) => write!(f, "Identifier"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Identifier {
	Print,
	Pascal,
}

pub const IDENTIFIER_SYMBOLS: &[(&str, Identifier)] = &[
	("print", Identifier::Print),
	("pascal", Identifier::Pascal),
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Literal {
	Integer(i32)
}

pub const TOKEN_SYMBOLS: &[(&str, Token)] = &[
	("+", Token::Plus),
	("-", Token::Minus),
	("*", Token::Asterisk),
	("/", Token::Slash),
	(";", Token::Semicolon),
];