use crate::parsing::Parser;
use crate::scanning::token::*;

#[derive(Debug)]
pub struct Generator {
	parser: Parser,
}