use crate::scanning::token::{Token, Literal};

#[derive(Debug)]
pub enum ASTNode {
	Literal(Literal),
	Binary { token: Token, left: Box<ASTNode>, right: Box<ASTNode> }
}