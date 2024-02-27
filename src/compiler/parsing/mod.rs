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
	(Token::GreaterThan, 11),
	(Token::GreaterThanEqual, 11),
	(Token::LessThan, 11),
	(Token::LessThanEqual, 11),
	(Token::Equals2, 10),
	(Token::ExclamationEqual, 10),
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
		dbg!(&token);

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
				_ => {Err(Error::IdentifierExpected { received: t })},
			},
			None => Err(Error::IdentifierExpected { received: Token::None })
		}
	}

	// Verify that current token matches expected identifier
	pub fn expect_identifier(&mut self, identifier: Identifier) -> Result<()> {
		let matched_identifier = self.match_identifier()?;

		if std::mem::discriminant(&matched_identifier) == std::mem::discriminant(&identifier) { Ok(()) } else { Err(Error::InvalidIdentifier { expected: [identifier].to_vec(), received: matched_identifier })}
	}

	// Parse a global statement (function for now)
	pub fn parse_global_statement(&mut self) -> Result<Option<ASTNode>> {
		if self.match_token(&[Token::EndOfFile]).is_ok() {
			return Ok(None);
		}

		// Should follow 'fn <name>(<param 1>, <param 2>, ...) { <body_block> }
		self.expect_identifier(Identifier::Function)?;
		self.scan_next()?;

		self.expect_identifier(Identifier::Symbol("".to_string()))?;
		let name = match self.match_identifier()? {
			Identifier::Symbol(s) => s,
			_ => "".to_string(),
		};
		self.scan_next()?;

		self.match_token(&[Token::LeftParen])?;
		self.scan_next()?;

		let mut param_list: Vec<FunctionParameter> = Vec::new();

		// Parse parameters until right parenthesis is met; don't allow trailing comma
		while self.expect_identifier(Identifier::Symbol("".to_string())).is_ok() {
			let id = self.match_identifier()?;
			self.scan_next()?;

			if !self.match_token(&[Token::RightParen]).is_ok() {
				self.match_token(&[Token::Comma])?;
				self.scan_next()?;
			}

			// Add id to parameter list (guaranteed to run)
			if let Identifier::Symbol(s) = id {
				param_list.push(FunctionParameter { name: s, param_type: Type::Named { type_name: "int".to_string() }});
			}
		}

		// Should be a right parenthesis
		self.match_token(&[Token::RightParen])?;
		self.scan_next()?;

		let body_block: Vec<ASTNode> = self.parse_block_statement()?;

		Ok(Some(ASTNode::FunctionDefinition { name, parameters: param_list, body_block, return_type: Type::Named { type_name: "int".to_string() } }))
	}

	// Parse a statement, which for now contains an identifier followed by a binary expression followed by a semicolon
	pub fn parse_statement(&mut self) -> Result<Option<ASTNode>> {
		// If EOF, None should be returned
		if self.match_token(&[Token::EndOfFile, Token::RightCurly]).is_ok() {
			return Ok(None);
		}

		// Statement should follow the pattern "<identifier> <binary_expr> ;"
		let identifier = self.match_identifier()?;
		self.scan_next()?;

		Ok(Some(match &identifier {
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
									val_type: None,
									value: val
								})
							},
							_ => Ok(ASTNode::Let { name: symbol, val_type: None, value: None })
						}
					},
					_ => Err(Error::InvalidIdentifier { expected: [Identifier::Symbol("".to_string())].to_vec(), received: id })
				}
			},
			Identifier::If => {
				// Follows 'if <expr> <block>'
				// Should get a boolean expression after if;
				let expr = Box::new(self.parse_binary_operation(0)?);

				// Parse a block statement and error if there isn't one
				let block = self.parse_block_statement()?;
				let is_else = self.expect_identifier(Identifier::Else).is_ok();
				if is_else {
					self.scan_next()?;
				}

				let else_block: Option<Vec<ASTNode>> = if is_else { Some(self.parse_block_statement()?) } else { None };

				Ok(ASTNode::If { expr, block, else_block })
			},
			Identifier::While => {
				// Follows 'while <expr> <block>'
				// Expecting boolean expression after keyword
				let expr = Box::new(self.parse_binary_operation(0)?);

				// Parse a block statement and error if there isn't one
				let block = self.parse_block_statement()?;

				Ok(ASTNode::While { expr, block })
			},
			Identifier::Return => {
				let return_val = Box::new(self.parse_binary_operation(0)?);
				self.scan_next()?;

				Ok(ASTNode::Return { return_val })
			},
			Identifier::Symbol(_) => {
				// Should match <symbol> = <value>;
				let token = self.match_token(&[Token::Equals])?;
				self.scan_next()?;

				let val = Box::new(self.parse_binary_operation(0)?);
				self.match_token(&[Token::Semicolon])?;
				self.scan_next()?;

				Ok(ASTNode::Binary { token, left: Box::new(ASTNode::Literal(Literal::Identifier(identifier.clone()))), right: val })
			},
			_ => Err(Error::InvalidIdentifier { received: identifier, expected: [Identifier::If, Identifier::Print, Identifier::Let, Identifier::Symbol("".to_string())].to_vec() }),
		}?))
	}

	// Parse a terminal node, i.e. a node is created with a literal token
	pub fn parse_terminal_node(&mut self) -> Result<ASTNode> {
		let Some(token) = self.current_token.clone() else {
			return Err(Error::LiteralExpected { received: Token::None });
		};

		match token {
			Token::Literal(Literal::Integer(x)) => {self.scan_next()?; Ok(ASTNode::Literal(Literal::Integer(x)))},
			Token::Literal(Literal::Identifier(Identifier::Symbol(c))) => {
				self.scan_next()?;

				// If a left parentheses follows, parse a function call
				if self.match_token(&[Token::LeftParen]).is_ok() {
					let mut arg_list: Vec<ASTNode> = Vec::new();
					self.scan_next()?;

					while self.match_token(&[Token::RightParen]).is_err() {
						let expr = self.parse_binary_operation(0)?;
						arg_list.push(expr);

						if self.match_token(&[Token::RightParen]).is_err() {
							self.match_token(&[Token::Comma])?;
							self.scan_next()?;
						}
					}
					self.scan_next()?;

					Ok(ASTNode::FunctionCall { name: c, args: arg_list })
				} else {
					Ok(ASTNode::Literal(Literal::Identifier(Identifier::Symbol(c))))
				}
			}
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

		let expr_finishers = [Token::Semicolon, Token::LeftCurly, Token::RightCurly, Token::RightParen, Token::Comma];

		if let Token::EndOfFile = token {
			return Err(Error::InvalidToken { expected: expr_finishers.to_vec(), received: Token::EndOfFile });
		} 
		
		if self.match_token(&expr_finishers).is_ok() {
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
			}
			
			if self.match_token(&expr_finishers).is_ok() {
				return Ok(left);
			}

			match &self.current_token {
				Some(t) => { token = t.clone(); },
				None => { return Err(Error::BinaryOperatorExpected { received: Token::None }); }
			}
		}

		Ok(left)
	}

	pub fn parse_block_statement(&mut self) -> Result<Vec<ASTNode>> {
		// Follows '{ <statement> <statement> ... }'
		self.match_token(&[Token::LeftCurly])?;
		self.scan_next()?;
		let mut statements = Vec::new();

		while self.current_token != Some(Token::RightCurly) {
			let statement = self.parse_statement()?;
			if let Some(node) = statement {
				statements.push(node);
			} else {
				return Err(Error::UnexpectedEOF { expected: Token::RightCurly });
			}
		}

		self.scan_next()?;

		Ok(statements)
	}
}