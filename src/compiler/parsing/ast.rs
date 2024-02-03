use crate::scanning::token::{Token};

#[derive(Debug)]
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

	pub fn new_from_children(token: Token, left: ASTNode, right: ASTNode) -> Self {
		Self {
			token,
			left: Some(Box::new(left)),
			right: Some(Box::new(right))
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