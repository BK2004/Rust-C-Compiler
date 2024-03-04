use std::{fmt, result};

use crate::parsing::ast::Type;
use crate::scanning::token::{Identifier, Token};
use crate::generating::llvm::{FunctionSignature, LLVMValue, RegisterFormat};

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
	FileOpenError { cause: std::io::Error },
	FileReadError { cause: std::io::Error },
	FileWriteError { cause: std::io::Error },
	InvalidToken { expected: Vec<Token>, received: Token },
	InvalidIdentifier { expected: Vec<Identifier>, received: Identifier },
	TerminalTokenExpected { received_token: Option<Token>, received_identifier: Option<Identifier> },
	UnknownToken { received: String },
	UnknownIdentifier { received: String },
	BinaryOperatorExpected { received: Token},
	IdentifierExpected { received: Token },
	LiteralExpected { received: Token },
	UnexpectedEOF { expected: Token },
	UnexpectedLLVMValue { expected: LLVMValue, received: LLVMValue },
	StringParseError { cause: std::num::ParseIntError },
	SymbolUndefined { name: String },
	SymbolDeclared { name: String },
	StatementExpected,
	ExpressionExpected,
	InvalidArithmeticOperand { received: RegisterFormat },
	InvalidComparisonOperands { left: RegisterFormat, right: RegisterFormat },
	InvalidAssignment { received: RegisterFormat, expected: RegisterFormat },
	TypeUnknown { received: Type },
	TypeExpected { received: Identifier },
	ArgumentMismatch { expected: FunctionSignature, received: Vec<LLVMValue> },
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::FileOpenError { cause } => write!(f, "FileOpenError: {cause}"),
			Error::FileReadError { cause } => write!(f, "FileReadError: {cause}"),
			Error::FileWriteError { cause } => write!(f, "FileWriteError: {cause}"),
			Error::InvalidToken { expected, received } => {
				let mut expected_concat = String::from("");
				for (i, t) in expected.iter().enumerate() {
					expected_concat += &format!("{t}");
					if i < expected.len() - 1 {
						expected_concat += ", ";
					}
				}

				if expected.len() > 1 {
					expected_concat.insert(0, '[');
					expected_concat.push(']');
				}

				write!(f, "InvalidToken: Expected {expected_concat}; got {received}")
			},
			Error::InvalidIdentifier { expected, received } => {
				let mut expected_concat = String::from("");
				for (i, id) in expected.iter().enumerate() {
					expected_concat += &format!("{id}");
					if i < expected.len() - 1 {
						expected_concat += ", ";
					}
				}

				if expected.len() > 1 {
					expected_concat.insert(0, '[');
					expected_concat.push(']');
				}

				write!(f, "InvalidToken: Expected {expected_concat}; got {received}")
			},
			Error::UnknownToken { received } => write!(f, "UnknownToken: {received}"),
			Error::UnknownIdentifier { received } => write!(f, "UnknownIdentifier: {received}"),
			Error::BinaryOperatorExpected { received } => write!(f, "BinaryOperatorExpected: Expected a binary operator, but got {received}"),
			Error::IdentifierExpected { received } => write!(f, "IdentifierExpected: Expected an identifier, but got {received}"),
			Error::LiteralExpected { received } => write!(f, "LiteralExpected: Expected a Literal, but received {received}"),
			Error::UnexpectedEOF { expected } => write!(f, "UnexpectedEOF: Expected {expected}, but reached EOF"),
			Error::UnexpectedLLVMValue { expected, received } => write!(f, "UnexpectedLLVMValue: Expected a {expected}, but received {received}"),
			Error::StringParseError { cause } => write!(f, "StringParseError: {cause}"),
			Error::SymbolUndefined { name } => write!(f, "SymbolUndefined: '{name}'"),
			Error::SymbolDeclared { name } => write!(f, "SymbolDeclared: Symbol {name} has already been declared"),
			Error::StatementExpected => write!(f, "StatementExpected: A statement was expected"),
			Error::ExpressionExpected => write!(f, "ExpressionExpected: An expression was expected"),
			Error::TerminalTokenExpected { received_token, received_identifier } => {
				if let Some(t) = received_token {
					write!(f, "TerminalTokenExpected: Expected a terminal token, but got {t} instead")
				} else if let Some(i) = received_identifier {
					write!(f, "TerminalTokenExpected: Expected a terminal token, but got {i} instead")
				} else {
					write!(f, "TerminalTokenExpected")
				}
			},
			Error::InvalidArithmeticOperand { received } => write!(f, "InvalidArithmeticOperand: Attempted to perform arithmetic on {received}"),
			Error::InvalidComparisonOperands { left, right } => write!(f, "InvalidComparisonOperands: Attempted to compare {left} and {right}"),
			Error::InvalidAssignment { received, expected } => write!(f, "InvalidAssigment: Attempted to assign {received} to {expected}"),
			Error::TypeUnknown { received } => write!(f, "TypeUnknown: '{received}'"),
			Error::TypeExpected { received } => write!(f, "TypeExpected: Expected a type, but got {received}"),
			Error::ArgumentMismatch { expected, received } => {
				write!(f, "ArgumentMismatch: Expected {expected}, but received (")?;

				for (i, rec) in received.iter().enumerate() {
					write!(f, "{fmt}", fmt=rec.format())?;
					if i < received.len() - 1 {
						write!(f, ",")?;
					}
				}

				write!(f, ")")
			}
		}
	}
}