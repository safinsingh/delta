use crate::{lexer::Lexer, parser::Parser};

use std::{io, io::Write};

const REPL_CHAR: &str = "◭ ";
const REPL_VERSION: &str = "0.1.1";

pub(crate) fn repl() -> io::Result<()> {
	println!("Delta v{} REPL", REPL_VERSION);
	println!("Type `exit` to exit.");

	loop {
		print!("{}", REPL_CHAR);
		io::stdout().flush()?;

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;

		match input.trim() {
			"exit" => break,
			_ => {
				let tok_stream = Lexer::new(input.trim());
				let stack = Parser::new(tok_stream).parse();
			}
		}
	}

	Ok(())
}
