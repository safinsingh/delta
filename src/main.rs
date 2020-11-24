use std::{env, fs, io};

mod lexer;
mod repl;

use lexer::Lexer;

fn main() -> io::Result<()> {
	let mut args: Vec<String> = env::args().collect();

	match args.len() {
		1 => {
			repl::repl()?;
		}
		_ => {
			args.remove(0);
			for file in args {
				let content = fs::read_to_string(&file)?;
				let lex = Lexer::new(&content);

				for lexed in lex {
					println!("{:?}", lexed)
				}
			}
		}
	}

	Ok(())
}
