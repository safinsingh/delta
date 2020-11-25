#![allow(clippy::unnecessary_wraps)]

use std::{env, fs, io};

mod lexer;
mod parser;
mod repl;

use lexer::Lexer;
use parser::{gen_parse_tree, Parser};

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
				let tok_stream = Lexer::new(&content);
				let stack = Parser::new(tok_stream).parse();
				let tree = gen_parse_tree(stack);

				println!("{:#?}", tree);
			}
		}
	}

	Ok(())
}
