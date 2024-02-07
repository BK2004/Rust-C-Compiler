#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Token {
	EndOfFile,
	Invalid,
	Literal(Literal),
	Plus,
	Minus,
	Asterisk,
	Slash,
	Semicolon,
	Identifier(Identifier),
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