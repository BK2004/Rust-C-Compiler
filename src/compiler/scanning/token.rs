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
	Equals2,
	ExclamationEqual,
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual,
}

impl Token {
	pub fn is_comparison(&self) -> bool {
		match self {
			Token::Equals2 | Token::ExclamationEqual | Token::LessThan | Token::LessThanEqual | Token::GreaterThan | Token::GreaterThanEqual => true,
			_ => false,
		}
	}

	pub fn get_pnemonic(&self) -> String {
		match self {
			Token::Equals2 => String::from("eq"),
			Token::ExclamationEqual => String::from("ne"),
			Token::LessThan => String::from("slt"),
			Token::LessThanEqual => String::from("sle"),
			Token::GreaterThan => String::from("sgt"),
			Token::GreaterThanEqual => String::from("sge"),
			_ => String::from(""),
		}
	}
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
			Token::Equals2 => write!(f, "=="),
			Token::ExclamationEqual => write!(f, "!="),
			Token::LessThan => write!(f, "<"),
			Token::LessThanEqual => write!(f, "<="),
			Token::GreaterThan => write!(f, ">"),
			Token::GreaterThanEqual => write!(f, ">="),
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
	("==", Token::Equals2),
	("!=", Token::ExclamationEqual),
	("<", Token::LessThan),
	("<=", Token::LessThanEqual),
	(">", Token::GreaterThan),
	(">=", Token::GreaterThanEqual),
];