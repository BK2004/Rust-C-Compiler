#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	EndOfFile,
	Invalid,
	IntegerLiteral(i32),
	Plus,
	Minus,
	Asterisk,
	Slash
}

pub const TOKEN_SYMBOLS: &[(&str, Token)] = &[
	("+", Token::Plus),
	("-", Token::Minus),
	("*", Token::Asterisk),
	("/", Token::Slash)
];