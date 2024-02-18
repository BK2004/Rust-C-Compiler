use crate::scanning::token::{Token, Literal};

#[derive(Debug, Clone)]
pub enum ASTNode {
	Literal(Literal),
	Binary { token: Token, left: Box<ASTNode>, right: Box<ASTNode> },
	Print {
		expr: Box<ASTNode>,
	},
	Let { 
		name: String,
		value: Option<Box<ASTNode>>,
	}
}