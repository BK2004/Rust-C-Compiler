use std::{fmt, result};

use crate::scanning::token::Token;
use crate::generating::llvm::LLVMValue;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
	FileOpenError { cause: std::io::Error },
	FileReadError { cause: std::io::Error },
	FileWriteError { cause: std::io::Error },
	InvalidToken { expected: Vec<Token>, received: Token },
	UnknownToken { received: String },
	UnknownIdentifier { received: String },
	BinaryOperatorExpected { received: Token},
	IdentifierExpected { received: Token },
	LiteralExpected { received: Token },
	UnexpectedEOF { expected: Token },
	UnexpectedLLVMValue { expected: LLVMValue, received: LLVMValue },
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

				write!(f, "InvalidToken: Expected {expected_concat}; got {received}")
			},
			Error::UnknownToken { received } => write!(f, "UnknownToken: {received}"),
			Error::UnknownIdentifier { received } => write!(f, "UnknownIdentifier: {received}"),
			Error::BinaryOperatorExpected { received } => write!(f, "BinaryOperatorExpected: Expected a binary operator, but got {received}"),
			Error::IdentifierExpected { received } => write!(f, "IdentifierExpected: Expected an identifier, but got {received}"),
			Error::LiteralExpected { received } => write!(f, "LiteralExpected: Expected a Literal, but received {received}"),
			Error::UnexpectedEOF { expected } => write!(f, "UnexpectedEOF: Expected {expected}, but reached EOF"),
			Error::UnexpectedLLVMValue { expected, received } => write!(f, "UnexpectedLLVMValue: Expected a {expected}, but received {received}"),
		}
	}
}