use std::io::Result
use crate::scanning::Scanner;
use crate::scanning::token::*;
use ast::*;

pub mod ast;

#[derive(Debug)]
pub enum Precedence {

}

pub struct Parser {
	scanner: Scanner,
	current_token: Option<Token>,
}

impl Parser {
	pub fn new(mut scanner: Scanner) -> Result<Self> {
		let token = scanner.scan()?;
		Ok(Self {
			scanner,
			current_token: token
		})
	}

	// Parse a terminal node, i.e. a node is created with a literal token
	pub fn parse_terminal_node(&mut self) -> Result<Option<ASTNode>> {
		if let None = self.current_token {
			return Ok(None);
		}

		let Some(token) = self.current_token;
		match token {
			Token::IntegerLiteral(_) => {self.current_token = self.scanner.scan()?; Ok(Some(ASTNode::new(token)))},
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
		}
	}

	pub fn parse_binary_operation(&mut self, precedence: u8) -> ASTNode {
		let left = self.parse_terminal_node();

		if let Some(Token::EndOfFile) == self.current_token {
			return left;
		}


	}
}