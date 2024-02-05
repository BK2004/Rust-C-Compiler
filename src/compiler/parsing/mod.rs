use std::io::Result;
use crate::scanning::Scanner;
use crate::scanning::token::*;
use ast::*;

pub mod ast;

pub const OPERATOR_PRECEDENCE: &[(Token, u8)] = &[
	(Token::Slash, 12),
	(Token::Asterisk, 12),
	(Token::Plus, 11),
	(Token::Minus, 11),
];

#[derive(Debug)]
pub struct Parser {
	scanner: Scanner,
	current_token: Option<Token>,
}

impl Parser {
	pub fn new(scanner: Scanner) -> Result<Self> {
		let mut parser = Self {
			scanner,
			current_token: None,
		};

		parser.scan_next()?;
		Ok(parser)
	}

	// Scan next token into parser
	pub fn scan_next(&mut self) -> Result<()> {
		let token = self.scanner.scan()?;

		self.current_token = token;
		Ok(())
	}

	// Parse a terminal node, i.e. a node is created with a literal token
	pub fn parse_terminal_node(&mut self) -> Result<ASTNode> {
		let Some(token) = self.current_token.clone() else {
			return Err(std::io::Error::new(std::io::ErrorKind::Other, "Terminal token expected"));
		};

		match token {
			Token::Literal(Literal::Integer(x)) => {self.current_token = self.scanner.scan()?; Ok(ASTNode::Literal(Literal::Integer(x)))},
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
		}
	}

	// Get precedence of token or error if not a valid operator
	pub fn get_precedence(&self, token: &Token) -> Result<u8> {
		// Search precedence array for token, else invalid token
		for (_, prec) in OPERATOR_PRECEDENCE.iter().enumerate() {
			if prec.0 == *token {
				return Ok(prec.1);
			}
		}
		Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
	}

	pub fn parse_binary_operation(&mut self, mut prev: u8) -> Result<ASTNode> {
		let mut left = self.parse_terminal_node()?;
		let mut right: ASTNode;

		let mut token: Token;

		match &self.current_token {
			Some(t) => { token = t.clone(); },
			None => { return Err(std::io::Error::new(std::io::ErrorKind::Other, "Token expected, got None")); }
		}

		if let Token::EndOfFile = token {
			return Ok(left);
		}

		while self.get_precedence(&token)? > prev {
			// Scan next token
			self.scan_next()?;

			// Recursively parse right of statement
			right = self.parse_binary_operation(self.get_precedence(&token)?)?;

			// Join left and right into parent node connected by operator token
			left = ASTNode::Binary { token: token.clone(), left: Box::new(left), right: Box::new(right), };

			// If EOF reached, return the new left
			if let Some(Token::EndOfFile) = self.current_token {
				return Ok(left);
			}

			match &self.current_token {
				Some(t) => { token = t.clone(); },
				None => { return Err(std::io::Error::new(std::io::ErrorKind::Other, "Token expected, got None")); }
			}
		}

		Ok(left)
	}
}