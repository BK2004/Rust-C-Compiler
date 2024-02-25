use crate::scanning::token::{Token, Literal};

#[derive(Debug, Clone)]
pub enum Type {
	Named {
		type_name: String
	}
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Named { type_name } => write!(f, "{type_name}"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
	pub name: String,
	pub param_type: Type,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
	Literal(Literal),
	Binary { token: Token, left: Box<ASTNode>, right: Box<ASTNode> },
	Print {
		expr: Box<ASTNode>,
	},
	Let { 
		name: String,
		val_type: Option<Type>,
		value: Option<Box<ASTNode>>,
	},
	If {
		expr: Box<ASTNode>,
		block: Vec<ASTNode>,
		else_block: Option<Vec<ASTNode>>,
	},
	While {
		expr: Box<ASTNode>,
		block: Vec<ASTNode>
	},
	FunctionDefinition {
		name: String,
		parameters: Vec<FunctionParameter>,
		body_block: Vec<ASTNode>,
		return_type: Type,
	}
}