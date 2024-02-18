use clap::Parser;
use crate::error::*;

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

		let scanner = crate::scanning::Scanner::open_file(filename.to_owned())?;
		let mut parser = crate::parsing::Parser::new(scanner)?;

		let mut generator = crate::generating::Generator::from_filename(filename.to_owned() + ".ll")?;
		generator.generate(&mut parser)?;
	}

	Ok(())
}