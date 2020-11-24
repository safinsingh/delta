use crate::lexer::Lexer;
use std::{io, io::Write};

const REPL_CHAR: &str = "◭ ";
const REPL_VERSION: &str = "0.1.0";

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
				let lex = Lexer::new(input.trim());
				for lexed in lex {
					println!("{:?}", lexed)
				}
			}
		}
	}

	Ok(())
}
