use crate::{
	ast,
	lexer::{Lexer, Token, TokenKind},
};

pub struct Parser<'a> {
	tokens: Lexer<'a>,
	pub op_stack: Vec<Token>,
}

impl<'a> Parser<'a> {
	pub fn new(tokens: Lexer<'a>) -> Parser<'a> {
		Self {
			tokens,
			op_stack: Vec::new(),
		}
	}

	pub fn parse(&mut self) {
		let mut out_stack = Vec::new();

		for token in self.tokens {
			if !token.is_op() {
				out_stack.push(token);
			} else if token.kind != TokenKind::LParen
				&& token.kind != TokenKind::RParen
			{
				// while op stack is not empty AND (op on top of the stack has
				// greater precedence than current token OR (the operator on the
				// top of the op stack has greater precedence than the current
				// token OR (the operator on the top of the stack has an equal
				// precedence to the current token AND the current token is left
				// associative))) AND the top of the op stack is not a left
				// parentheses, DO pop operators from the operator stack onto
				// the output stack
				while self.op_stack.len() > 0
					&& (self.op_stack.last().map(|op| op.get_precedence())
						> Some(token.get_precedence())
						|| (self.op_stack.last().map(|op| op.get_precedence())
							== Some(token.get_precedence())
							&& token.get_associativity()
								== ast::Association::LTR))
					&& !matches!(
						self.op_stack.last(),
						Some(&Token {
							kind: TokenKind::LParen,
							..
						})
					) {
					out_stack.push(self.op_stack.pop().unwrap())
				}
				self.op_stack.push(token);
			} else if token.kind == TokenKind::LParen {
				self.op_stack.push(token);
			} else if token.kind == TokenKind::RParen {
				while !matches!(
					self.op_stack.last(),
					Some(&Token {
						kind: TokenKind::LParen,
						..
					})
				) {
					if let Some(token) = self.op_stack.pop() {
						out_stack.push(token);
					} else {
						break;
					}
				}
				if matches!(
					self.op_stack.last(),
					Some(&Token {
						kind: TokenKind::LParen,
						..
					})
				) {
					self.op_stack.pop();
				} else {
					panic!("Unmatched right parentheses: {:#?}", token);
				}
			}
		}

		while !self.op_stack.is_empty() {
			out_stack.push(self.op_stack.pop().unwrap());
		}
	}
}
