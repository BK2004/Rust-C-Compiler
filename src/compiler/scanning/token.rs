use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Token {
	EndOfFile,
	None,
	Literal(Literal),
	LeftCurly,
	RightCurly,
	LeftParen,
	RightParen,
	Plus,
	Minus,
	Asterisk,
	Slash,
	Semicolon,
	Comma,
	Colon,
	Equals,
	Equals2,
	ExclamationEqual,
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual,
	Arrow,
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
			Token::LeftCurly => write!(f, "{{"),
			Token::RightCurly => write!(f, "}}"),
			Token::LeftParen => write!(f, "("),
			Token::RightParen => write!(f, ")"),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Asterisk => write!(f, "*"),
			Token::Slash => write!(f, "/"),
			Token::Semicolon => write!(f, ";"),
			Token::Colon => write!(f, ":"),
			Token::Comma => write!(f, ","),
			Token::Equals => write!(f, "="),
			Token::Equals2 => write!(f, "=="),
			Token::ExclamationEqual => write!(f, "!="),
			Token::LessThan => write!(f, "<"),
			Token::LessThanEqual => write!(f, "<="),
			Token::GreaterThan => write!(f, ">"),
			Token::GreaterThanEqual => write!(f, ">="),
			Token::Arrow => write!(f, "->"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
	Let,
	Print,
	If,
	Else,
	While,
	Function,
	Return,
	Symbol(String),
}

impl fmt::Display for Identifier {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Identifier::Print => write!(f, "print"),
			Identifier::Let => write!(f, "let"),
			Identifier::If => write!(f, "if"),
			Identifier::Else => write!(f, "else"),
			Identifier::While => write!(f, "while"),
			Identifier::Function => write!(f, "fn"),
			Identifier::Return => write!(f, "return"),
			Identifier::Symbol(s) => write!(f, "{s}"),
		}
	}
}

pub const IDENTIFIER_SYMBOLS: &[(&str, Identifier)] = &[
	("let", Identifier::Let),
	("print", Identifier::Print),
	("if", Identifier::If),
	("else", Identifier::Else),
	("while", Identifier::While),
	("fn", Identifier::Function),
	("return", Identifier::Return),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
	Integer(i64),
	Identifier(Identifier)
}

pub const TOKEN_SYMBOLS: &[(&str, Token)] = &[
	("{", Token::LeftCurly),
	("}", Token::RightCurly),
	("(", Token::LeftParen),
	(")", Token::RightParen),
	("+", Token::Plus),
	("-", Token::Minus),
	("*", Token::Asterisk),
	("/", Token::Slash),
	(";", Token::Semicolon),
	(",", Token::Comma),
	(":", Token::Colon),
	("=", Token::Equals),
	("==", Token::Equals2),
	("!=", Token::ExclamationEqual),
	("<", Token::LessThan),
	("<=", Token::LessThanEqual),
	(">", Token::GreaterThan),
	(">=", Token::GreaterThanEqual),
	("->", Token::Arrow),
];