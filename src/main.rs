#![allow(clippy::unnecessary_wraps)]

//! The Delta Programming Language

use std::{env, fs, io};

mod ast;
mod lexer;
mod parser;
mod repl;

use lexer::Lexer;
use parser::Parser;

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
				let lexer = Lexer::new(content.trim());
				let stack = Parser::new(lexer).parse();

				println!("{:#?}", stack);
			}
		}
	}

	Ok(())
}
