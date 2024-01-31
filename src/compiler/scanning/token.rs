#[derive(Debug)]
pub enum Token {
	Invalid,
	IntegerLiteral(isize),
	Plus,
	Minus,
	Asterisk,
	Slash
}