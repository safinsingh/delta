use std::fmt;

#[derive(PartialEq, Debug)]
pub(crate) enum TokenKind {
	// Operators
	Plus,
	Minus,
	Multiply,
	Divide,
	Mod,
	LessEq,
	GreaterEq,
	Greater,
	Less,
	BitAnd,
	BitOr,
	BitNot,
	Xor,

	// Keywords
	True,
	False,
	Fun,
	Match,
	While,
	For,

	// Symbols
	LParen,
	RParen,
	LBrace,
	RBrace,
	LBracket,
	RBracket,
	Colon,
	Comma,
	Period,
	And,
	Or,
	Not,
	Assign,
	Equate,
	MatchArm,

	// Misc
	String(String),
	Number(f64),
	Ident(String),
	Undefined(String),
	Comment(String),
	Newline,
}

#[derive(PartialEq)]
pub(crate) struct Token {
	pub(crate) kind: TokenKind,
	span: (usize, usize),
}

impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"[TOK] ({}, {})\t{:?}",
			self.span.0, self.span.1, self.kind,
		)
	}
}

pub(crate) struct Lexer<'a> {
	input: &'a str,
	position: usize,
	cursor: (usize, usize),
}

impl<'a> Lexer<'a> {
	pub(crate) fn new(input: &'a str) -> Lexer<'a> {
		Self {
			input,
			position: 0,
			cursor: (1, 0),
		}
	}

	fn translate(&mut self, chars: usize) {
		self.cursor.1 += chars;
		self.position += chars;
	}

	fn single_char_token(&mut self, tok: TokenKind) -> Option<Token> {
		let pos = self.cursor;
		self.translate(1);

		Some(Token {
			kind: tok,
			span: pos,
		})
	}

	fn double_char_token(&mut self, tok: TokenKind) -> Option<Token> {
		let pos = self.cursor;
		self.translate(2);

		Some(Token {
			kind: tok,
			span: pos,
		})
	}

	fn get_char_raw(&self, pos: Option<usize>) -> Option<char> {
		let nth = |x| self.input.chars().nth(x);

		if let Some(p) = pos {
			nth(p)
		} else {
			nth(self.position)
		}
	}

	fn peek(&self) -> Option<char> {
		self.get_char_raw(Some(self.position + 1))
	}

	fn string(&mut self) -> Option<Token> {
		let mut str = String::new();
		let pos = self.cursor;

		self.translate(1);
		while let Some(ch) = self.get_char_raw(None) {
			match ch {
				'\\' if self.peek() == Some('"') => {
					str.push('"');
					self.translate(2);
				}
				'"' => {
					self.translate(1);
					break;
				}
				_ => {
					str.push(ch);
					self.translate(1);
				}
			}
		}

		Some(Token {
			kind: TokenKind::String(str),
			span: pos,
		})
	}

	fn identifier(&mut self) -> Option<Token> {
		let mut str = String::new();
		let pos = self.cursor;

		while let Some(ch) = self.get_char_raw(None) {
			match ch {
				'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
					str.push(ch);
					self.translate(1);
				}
				_ => break,
			}
		}

		let tok = match str.as_str() {
			"true" => TokenKind::True,
			"false" => TokenKind::False,
			"fun" => TokenKind::Fun,
			"match" => TokenKind::Match,
			"while" => TokenKind::While,
			"for" => TokenKind::For,
			_ => TokenKind::Ident(str),
		};

		Some(Token {
			kind: tok,
			span: pos,
		})
	}

	fn whitespace(&mut self) -> Option<Token> {
		self.translate(1);
		self.next()
	}

	fn newline(&mut self) -> Option<Token> {
		let token = Token {
			kind: TokenKind::Newline,
			span: self.cursor,
		};

		self.translate(1);
		self.cursor.0 += 1;
		self.cursor.1 = 0;

		Some(token)
	}

	fn number(&mut self) -> Option<Token> {
		let mut str = String::new();
		let pos = self.cursor;

		while let Some(ch) = self.get_char_raw(None) {
			match ch {
				'0'..='9' => {
					str.push(ch);
					self.translate(1);
				}
				'.' if matches!(self.peek(), Some('0'..='9')) => {
					str.push(ch);
					self.translate(1);
				}
				_ => break,
			}
		}

		match str.parse() {
			Ok(num) => Some(Token {
				kind: TokenKind::Number(num),
				span: pos,
			}),
			_ => Some(Token {
				kind: TokenKind::Undefined(str),
				span: pos,
			}),
		}
	}

	fn comment(&mut self) -> Option<Token> {
		let mut str = String::new();
		let pos = self.cursor;

		self.translate(2);
		while let Some(ch) = self.get_char_raw(None) {
			match ch {
				'\n' => {
					break;
				}
				_ => {
					str.push(ch);
					self.translate(1);
				}
			}
		}

		Some(Token {
			kind: TokenKind::Comment(str.trim().into()),
			span: pos,
		})
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		let current_char = self.get_char_raw(None);
		let next_char = self.get_char_raw(Some(self.position + 1));

		if current_char == None {
			return None;
		}
		match current_char.unwrap() {
			'\0' => None,
			'/' if next_char == Some('/') => self.comment(),
			'"' => self.string(),
			'\n' => self.newline(),
			' ' | '\t' => self.whitespace(),
			'=' if next_char == Some('=') => {
				self.double_char_token(TokenKind::Equate)
			}
			'=' => self.single_char_token(TokenKind::Assign),
			'(' => self.single_char_token(TokenKind::LParen),
			')' => self.single_char_token(TokenKind::RParen),
			'{' => self.single_char_token(TokenKind::LBrace),
			'}' => self.single_char_token(TokenKind::RBrace),
			'[' => self.single_char_token(TokenKind::LBracket),
			']' => self.single_char_token(TokenKind::RBracket),
			'-' if next_char == Some('>') => {
				self.double_char_token(TokenKind::MatchArm)
			}
			'+' => self.single_char_token(TokenKind::Plus),
			'-' => self.single_char_token(TokenKind::Minus),
			'*' => self.single_char_token(TokenKind::Multiply),
			'/' => self.single_char_token(TokenKind::Divide),
			'%' => self.single_char_token(TokenKind::Mod),
			'<' if next_char == Some('=') => {
				self.double_char_token(TokenKind::LessEq)
			}
			'>' if next_char == Some('=') => {
				self.double_char_token(TokenKind::GreaterEq)
			}
			'<' => self.single_char_token(TokenKind::Less),
			'>' => self.single_char_token(TokenKind::Greater),
			'&' if next_char == Some('&') => {
				self.double_char_token(TokenKind::And)
			}
			'|' if next_char == Some('|') => {
				self.double_char_token(TokenKind::Or)
			}
			'&' => self.single_char_token(TokenKind::BitAnd),
			'|' => self.single_char_token(TokenKind::BitOr),
			'~' => self.single_char_token(TokenKind::BitNot),
			'^' => self.single_char_token(TokenKind::Xor),
			'!' => self.single_char_token(TokenKind::Not),
			':' => self.single_char_token(TokenKind::Colon),
			',' => self.single_char_token(TokenKind::Comma),
			'.' => self.single_char_token(TokenKind::Period),
			'A'..='Z' | 'a'..='z' => self.identifier(),
			'0'..='9' => self.number(),
			_ => self.single_char_token(TokenKind::Undefined(
				current_char.unwrap().into(),
			)),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn lex_newline() {
		let input = "\n";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Newline,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_comment() {
		let input = "//hello world!\n// test!";
		let lexer = Lexer::new(input).collect::<Vec<_>>();

		assert_eq!(
			vec![
				Token {
					kind: TokenKind::Comment("hello world!".into()),
					span: (1, 0)
				},
				Token {
					kind: TokenKind::Newline,
					span: (1, 14)
				},
				Token {
					kind: TokenKind::Comment("test!".into()),
					span: (2, 0)
				},
			],
			lexer
		)
	}

	#[test]
	fn lex_string() {
		let input = " \"hello!\"";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::String("hello!".into()),
				span: (1, 1)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_whitespace() {
		let input = "     \t   \t";
		let mut lexer = Lexer::new(input);

		assert_eq!(None, lexer.next())
	}

	#[test]
	fn lex_equate() {
		let input = "  ==";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Equate,
				span: (1, 2)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_assign() {
		let input = " =";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Assign,
				span: (1, 1)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_parens() {
		let input = "()";
		let lexer = Lexer::new(input).collect::<Vec<_>>();

		assert_eq!(
			vec![
				Token {
					kind: TokenKind::LParen,
					span: (1, 0)
				},
				Token {
					kind: TokenKind::RParen,
					span: (1, 1)
				}
			],
			lexer
		)
	}

	#[test]
	fn lex_braces() {
		let input = "{}";
		let lexer = Lexer::new(input).collect::<Vec<_>>();

		assert_eq!(
			vec![
				Token {
					kind: TokenKind::LBrace,
					span: (1, 0)
				},
				Token {
					kind: TokenKind::RBrace,
					span: (1, 1)
				}
			],
			lexer
		)
	}

	#[test]
	fn lex_brackets() {
		let input = "[]";
		let lexer = Lexer::new(input).collect::<Vec<_>>();

		assert_eq!(
			vec![
				Token {
					kind: TokenKind::LBracket,
					span: (1, 0)
				},
				Token {
					kind: TokenKind::RBracket,
					span: (1, 1)
				}
			],
			lexer
		)
	}

	#[test]
	fn lex_match_arms() {
		let input = "-> ->";
		let lexer = Lexer::new(input).collect::<Vec<_>>();

		assert_eq!(
			vec![
				Token {
					kind: TokenKind::MatchArm,
					span: (1, 0)
				},
				Token {
					kind: TokenKind::MatchArm,
					span: (1, 3)
				}
			],
			lexer
		)
	}

	#[test]
	fn lex_plus() {
		let input = "+";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Plus,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_minus() {
		let input = "-";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Minus,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_multiply() {
		let input = "*";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Multiply,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_divide() {
		let input = "/";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Divide,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_mod() {
		let input = "%";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Mod,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_greater() {
		let input = ">";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Greater,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_less() {
		let input = "<";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Less,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_greater_equal() {
		let input = ">=";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::GreaterEq,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_less_equal() {
		let input = "<=";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::LessEq,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_bitwise_or() {
		let input = "|";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::BitOr,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_bitwise_and() {
		let input = "&";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::BitAnd,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_bitwise_not() {
		let input = "~";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::BitNot,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_logical_or() {
		let input = "||";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Or,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_logical_and() {
		let input = "&&";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::And,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_logical_not() {
		let input = "!";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Not,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_colon() {
		let input = ":";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Colon,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_comma() {
		let input = ",";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Comma,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_period() {
		let input = ".";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Period,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_ident() {
		let input = "br_uh";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Ident("br_uh".into()),
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_true() {
		let input = "true";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::True,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_false() {
		let input = "false";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::False,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_fun() {
		let input = "fun";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Fun,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_match() {
		let input = "match";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Match,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_while() {
		let input = "while";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::While,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_for() {
		let input = "for";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::For,
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_float() {
		let input = "  123.03";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Number(123.03),
				span: (1, 2)
			},
			lexer.next().unwrap()
		)
	}

	#[test]
	fn lex_int() {
		let input = "142 ";
		let mut lexer = Lexer::new(input);

		assert_eq!(
			Token {
				kind: TokenKind::Number(142.),
				span: (1, 0)
			},
			lexer.next().unwrap()
		)
	}
}
