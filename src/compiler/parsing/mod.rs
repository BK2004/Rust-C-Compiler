use crate::error::*;
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

	// Verify that token matches what is expected
	pub fn match_token(&mut self, tokens: &[Token]) -> Result<Token> {
		for (_, token) in tokens.iter().enumerate() {
			let matched = match self.current_token.clone() {
				Some(t) => *token == t,
				None => false,
			};

			if matched {
				return Ok(self.current_token.clone().unwrap());
			}
		}

		// Error handling
		let t: Token;
		if let Some(tok) = self.current_token.clone() { t = tok; } else {t = Token::None};
		Err(Error::InvalidToken { expected: tokens.to_vec(), received: t })
	}

	// Verify that current token matches an identifier and return said identifier
	pub fn match_identifier(&mut self) -> Result<Identifier> {
		match self.current_token.clone() {
			Some(t) => match t {
				Token::Literal(Literal::Identifier(i)) => Ok(i),
				_ => Err(Error::IdentifierExpected { received: t }),
			},
			None => Err(Error::IdentifierExpected { received: Token::None })
		}
	}

	// Parse a statement, which for now contains an identifier followed by a binary expression followed by a semicolon
	pub fn parse_statement(&mut self) -> Result<Option<ASTNode>> {
		// If EOF, None should be returned
		if let Some(Token::EndOfFile) = self.current_token {
			return Ok(None)
		}

		// Statement should follow the pattern "<identifier> <binary_expr> ;"
		let identifier = self.match_identifier()?;
		self.scan_next()?;

		Ok(Some(match identifier {
			Identifier::Print => {
				Ok(ASTNode::Print {
					expr: {
						let b = Box::new(self.parse_binary_operation(0)?);
						self.match_token(&[Token::Semicolon])?;
						self.scan_next()?;

						b
					}
				})
			},
			Identifier::Let => {
				// Let should be formatted as either 'let <symbol> = <value>;' or 'let <symbol>;'
				let id = self.match_identifier()?;
				self.scan_next()?;

				match id {
					Identifier::Symbol(symbol) => {
						let eq_or_semi = self.match_token(&[Token::Equals, Token::Semicolon])?;
						self.scan_next()?;

						// If eq_or_semi is Equals, assigment should occur in the same line; else, statement ends on semicolon
						match eq_or_semi {
							Token::Equals => {
								let val = Some(Box::new(self.parse_binary_operation(0)?));
								self.match_token(&[Token::Semicolon])?;
								self.scan_next()?;
								Ok(ASTNode::Let {
									name: symbol,
									value: val
								})
							},
							_ => Ok(ASTNode::Let { name: symbol, value: None })
						}
					},
					_ => Err(Error::InvalidIdentifier { expected: [Identifier::Symbol("".to_string())].to_vec(), received: id })
				}
			},
			_ => Err(Error::InvalidIdentifier { received: identifier, expected: [Identifier::Print, Identifier::Let].to_vec() }),
		}?))
	}

	// Parse a terminal node, i.e. a node is created with a literal token
	pub fn parse_terminal_node(&mut self) -> Result<ASTNode> {
		let Some(token) = self.current_token.clone() else {
			return Err(Error::LiteralExpected { received: Token::None });
		};

		dbg!(&token);

		match token {
			Token::Literal(Literal::Integer(x)) => {self.current_token = self.scanner.scan()?; Ok(ASTNode::Literal(Literal::Integer(x)))},
			Token::Literal(Literal::Identifier(Identifier::Symbol(c))) => {self.scan_next()?; Ok(ASTNode::Literal(Literal::Identifier(Identifier::Symbol(c))))}
			_ => Err(Error::LiteralExpected { received: token })
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
		Err(Error::BinaryOperatorExpected { received: token.clone() })
	}

	pub fn parse_binary_operation(&mut self, prev: u8) -> Result<ASTNode> {
		let mut left = self.parse_terminal_node()?;
		let mut right: ASTNode;

		let mut token: Token;

		match &self.current_token {
			Some(t) => { token = t.clone(); },
			None => { return Err(Error::BinaryOperatorExpected { received: Token::None }); }
		}

		if let Token::EndOfFile = token {
			return Err(Error::InvalidToken { expected: [Token::Semicolon].to_vec(), received: Token::EndOfFile });
		} else if let Token::Semicolon = token {
			return Ok(left);
		}

		while self.get_precedence(&token)? > prev {
			// Scan next token
			self.scan_next()?;

			// Recursively parse right of statement
			right = self.parse_binary_operation(self.get_precedence(&token)?)?;

			// Join left and right into parent node connected by operator token
			left = ASTNode::Binary { token: token.clone(), left: Box::new(left), right: Box::new(right) };

			// If EOF reached, return the new left
			if let Some(Token::EndOfFile) = self.current_token {
				return Ok(left);
			} else if let Some(Token::Semicolon) = self.current_token {
				return Ok(left);
			}

			match &self.current_token {
				Some(t) => { token = t.clone(); },
				None => { return Err(Error::BinaryOperatorExpected { received: Token::None }); }
			}
		}

		Ok(left)
	}
}