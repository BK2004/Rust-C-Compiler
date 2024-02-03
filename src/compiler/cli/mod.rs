use clap::Parser;
use std::io::Result;
use crate::scanning::token::*;

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct Args {
	// Input files to be compiled
	#[arg(required=true)]
	input_files: Vec<String>,

	// Enable debug mode
	#[arg(short, long)]
	debug: bool,
}

impl Args {
	// Accessors
	pub fn input_files(&self) -> &Vec<String> {
		&self.input_files
	}

	pub fn debug(&self) -> bool {
		self.debug
	}
}

pub fn parse_args() -> Args {
	Args::parse()
}

pub fn compile(args: &Args) -> Result<()> {
	for (_, filename) in args.input_files().iter().enumerate() {
		if args.debug() {
			println!("Compiling {}.", filename);
		}

		let mut scanner = crate::scanning::Scanner::open_file(filename.clone())?;
		let mut parser = crate::parsing::Parser::new(scanner)?;

		println!("{}", interpret_ast(&parser.parse_binary_operation(0)?)?);
	}

	Ok(())
}

pub fn interpret_ast(node: &crate::parsing::ast::ASTNode) -> Result<i32> {
	if let Some(l) = node.left() {
		let left = interpret_ast(&**l)?;

		// Check for right child; if it's there, this is a binary operation
		if let Some(r) = node.right() {
			let right = interpret_ast(&**r)?;

			// Match binary operators
			return match node.token() {
				Token::Asterisk => Ok(left * right),
				Token::Minus => Ok(left - right),
				Token::Plus => Ok(left + right),
				Token::Slash => Ok(left / right),
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
			}
		} else { // Else this is unary (not yet implemented, so error)
			return match node.token() {
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token")),
			}
		}
	} else if let Some(r) = node.right() { // If ony right node is present, this is unary
		let right = interpret_ast(&**r)?;

		// Match unary operators (not implemented yet, so error)
		return match node.token() {
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
		}
	}

	// No node should have zero children (unless token is a literal)
	match node.token() {
		Token::IntegerLiteral(x) => Ok(*x),
		_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
	}
}