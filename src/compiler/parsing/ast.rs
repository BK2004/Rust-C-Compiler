use crate::scanning::token::{Token};

pub struct ASTNode {
	token: Token,
	left: Option<Box<ASTNode>>,
	right: Option<Box<ASTNode>>,
}

impl ASTNode {
	pub fn new(token: Token) -> Self {
		Self {
			token,
			left: None,
			right: None, 
		}
	}

	pub fn token(&self) -> &Token {
		&self.token
	}

	pub fn left(&self) -> &Option<Box<ASTNode>> {
		&self.left
	}

	pub fn right(&self) -> &Option<Box<ASTNode>> {
		&self.right
	}
}