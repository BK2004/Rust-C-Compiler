use compiler::cli;

fn main() {
    let args: cli::Args = cli::parse_args();
	if args.debug() {
		dbg!(&args);
	}

	match cli::compile(&args) {
		Err(error) => println!("Error: {}", error),
		Ok(_) => println!("Successfully compiled files!"),
	}
}
