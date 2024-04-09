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
	Ampersand,
	Let,
	Print,
	If,
	Else,
	While,
	Function,
	Return,
}

impl Token {
	pub fn is_rl_associativity(&self) -> bool {
		match self {
			Token::Equals => true,
			_ => false,
		}
	}

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
			Token::Ampersand => write!(f, "&"),
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
			Token::Print => write!(f, "print"),
			Token::Let => write!(f, "let"),
			Token::If => write!(f, "if"),
			Token::Else => write!(f, "else"),
			Token::While => write!(f, "while"),
			Token::Function => write!(f, "fn"),
			Token::Return => write!(f, "return"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
	Symbol(String),
}

impl fmt::Display for Identifier {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			Identifier::Symbol(s) => write!(f, "{s}"),
		}
	}
}

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
	("&", Token::Ampersand),
];

pub const KEYWORD_TOKENS: &[(&str, Token)] = &[
	("let", Token::Let),
	("print", Token::Print),
	("if", Token::If),
	("else", Token::Else),
	("while", Token::While),
	("fn", Token::Function),
	("return", Token::Return),
];