#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Token {
	EndOfFile,
	Invalid,
	Literal(Literal),
	Plus,
	Minus,
	Asterisk,
	Slash
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Literal {
	Integer(i32)
}

pub const TOKEN_SYMBOLS: &[(&str, Token)] = &[
	("+", Token::Plus),
	("-", Token::Minus),
	("*", Token::Asterisk),
	("/", Token::Slash)
];