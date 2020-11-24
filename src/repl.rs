use crate::lexer::Lexer;
use std::{io, io::Write};

pub(crate) fn repl() -> io::Result<()> {
	println!("Delta v0.1.0 REPL");
	println!("Type `exit` to exit.");

	Ok(loop {
		print!("◭ ");
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
	})
}
