use crate::scanning::token::{Token, Literal};

#[derive(Debug, Clone)]
pub enum Type {
	Named {
		type_name: String
	},
	Pointer {
		pointee_type: Box<Type>,
	},
	Void
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Named { type_name } => write!(f, "{type_name}"),
			Type::Pointer { pointee_type } => write!(f, "*{pointee_type}"),
			Type::Void => write!(f, "void"),
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
	},
	FunctionCall {
		name: String,
		args: Vec<ASTNode>,
	},
	Return {
		return_val: Option<Box<ASTNode>>,
	},
	Dereference {
		child: Box<ASTNode>,
	},
	Reference {
		child: Box<ASTNode>,
	}
}