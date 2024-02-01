use clap::Parser;
use crate::scanning::{token, Scanner};

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

pub fn compile(args: &Args) -> std::io::Result<()> {
	for (_, filename) in args.input_files().iter().enumerate() {
		if args.debug() {
			println!("Compiling {}.", filename);
		}

		let mut scanner = Scanner::open_file(filename.clone())?;

		while let Some(c) = scanner.scan()? {
			if c == token::Token::EndOfFile {
				break;
			}
			print!("{:?} ", c);
		}
		println!();
	}

	Ok(())
}