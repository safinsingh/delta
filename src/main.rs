use std::{env, fs, io, io::Write};

mod lexer;
use lexer::Lexer;

fn main() -> io::Result<()> {
	let mut args: Vec<String> = env::args().collect();

	match args.len() {
		1 => {
			println!("Delta v0.1.0 REPL");
			println!("Type `exit` to exit.");

			loop {
				print!("◭ ");
				if let Err(e) = io::stdout().flush() {
					panic!("Fatal error: failed to flush stdout: {}", e)
				}

				let mut input = String::new();
				io::stdin()
					.read_line(&mut input)
					.expect("error: unable to read user input");

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
