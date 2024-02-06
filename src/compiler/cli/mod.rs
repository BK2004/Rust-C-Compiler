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

		let scanner = crate::scanning::Scanner::open_file(filename.clone())?;
		let mut parser = crate::parsing::Parser::new(scanner)?;

		let mut generator = crate::generating::Generator::from_filename(filename.clone() + ".ll")?;
		generator.generate(&mut parser)?;

		// println!("{}", interpret_ast(&parser.parse_binary_operation(0)?)?);
	}

	Ok(())
}

pub fn interpret_ast(node: &crate::parsing::ast::ASTNode) -> Result<i32> {
	match node {
		crate::parsing::ast::ASTNode::Literal(Literal::Integer(x)) => Ok(*x),
		crate::parsing::ast::ASTNode::Binary{token, left, right} => {
			let left_res = interpret_ast(&left)?;
			let right_res = interpret_ast(&right)?;

			return match token {
				Token::Asterisk => Ok(left_res * right_res),
				Token::Minus => Ok(left_res - right_res),
				Token::Plus => Ok(left_res + right_res),
				Token::Slash => Ok(left_res / right_res),
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid token"))
			};
		}
	}
}