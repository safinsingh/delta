use crate::lexer::{Lexer, Token, TokenKind};

impl TokenKind {
	fn get_precedence(&self) -> u8 {
		match self {
			TokenKind::LParen | TokenKind::RParen => 12,
			TokenKind::Not | TokenKind::BitNot => 11,
			TokenKind::Multiply | TokenKind::Divide | TokenKind::Mod => 10,
			TokenKind::Plus | TokenKind::Minus => 9,
			TokenKind::Greater
			| TokenKind::GreaterEq
			| TokenKind::Less
			| TokenKind::LessEq => 8,
			TokenKind::Equate => 7,
			TokenKind::BitAnd => 6,
			TokenKind::Xor => 5,
			TokenKind::BitOr => 4,
			TokenKind::And => 3,
			TokenKind::Or => 2,
			TokenKind::Assign => 1,
			_ => 0,
		}
	}

	fn is_op(&self) -> bool { self.get_precedence() > 0 }

	fn is_bin_op(&self) -> bool {
		matches!(
			self,
			TokenKind::Multiply
				| TokenKind::Divide
				| TokenKind::Mod | TokenKind::Plus
				| TokenKind::Minus
				| TokenKind::Greater
				| TokenKind::GreaterEq
				| TokenKind::Less
				| TokenKind::LessEq
				| TokenKind::BitAnd
				| TokenKind::Xor | TokenKind::BitOr
				| TokenKind::And | TokenKind::Or
		)
	}
}

pub struct Parser<'a> {
	tokens: Lexer<'a>,
	op_stack: Vec<TokenKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
	BinExpr {
		op: TokenKind,
		lhs: Box<Node>,
		rhs: Box<Node>,
	},
	NumberLiteral(f64),
	IdentLiteral(String),
	StringLiteral(String),
}

impl<'a> Parser<'a> {
	pub fn new(tokens: Lexer<'a>) -> Parser<'a> {
		Self {
			tokens,
			op_stack: Vec::new(),
		}
	}

	pub fn parse(&mut self) -> Vec<TokenKind> {
		let mut out_stack = Vec::new();

		for token in self.tokens {
			if token.kind.is_op() {
				if self.op_stack.is_empty()
					|| self.op_stack.last() == Some(&TokenKind::LParen)
					|| token.kind == TokenKind::LParen
				{
					self.op_stack.push(token.kind);
				} else if token.kind == TokenKind::RParen {
					while self.op_stack.last() != Some(&TokenKind::LParen) {
						out_stack.push(self.op_stack.pop().unwrap());
					}
					self.op_stack.pop();
				} else {
					self.precedence_based_pushpop(&mut out_stack, token);
				}
			} else {
				out_stack.push(token.kind);
			}
		}
		for tok in self.op_stack.clone().into_iter().rev() {
			out_stack.push(tok);
		}
		// TODO: error handling: op stack should be empty (specifically, it
		// should be parenthesis-free)
		out_stack
	}

	fn precedence_based_pushpop(
		&mut self,
		out_stack: &mut Vec<TokenKind>,
		token: Token,
	) {
		if token.kind.is_op() {
			if let Some(last) = self.op_stack.last() {
				if token.kind.get_precedence() > last.get_precedence() {
					self.op_stack.push(token.kind);
				} else if token.kind.get_precedence() == last.get_precedence() {
					// if precedence is the same, compare associativity
				} else if token.kind.get_precedence() < last.get_precedence() {
					out_stack.push(self.op_stack.pop().unwrap());
					self.precedence_based_pushpop(out_stack, token);
				}
			} else {
				self.op_stack.push(token.kind);
			}
		} else {
			self.op_stack.push(token.kind);
		}
	}
}

pub fn gen_parse_tree(out_stack: Vec<TokenKind>) -> Box<Node> {
	let mut node_stack = Vec::new();

	for token in &out_stack {
		if token.is_bin_op() {
			let rhs = node_stack.pop().unwrap();
			let lhs = node_stack.pop().unwrap();

			node_stack.push(Box::from(Node::BinExpr {
				op: token.clone(),
				lhs,
				rhs,
			}))
		} else {
			match token.clone() {
				TokenKind::Ident(i) => {
					node_stack.push(Box::from(Node::IdentLiteral(i)))
				}
				TokenKind::Number(num) => {
					node_stack.push(Box::from(Node::NumberLiteral(num)))
				}
				TokenKind::String(s) => {
					node_stack.push(Box::from(Node::StringLiteral(s)))
				}
				_ => todo!("Unrecognized non-binary token!"),
			}
		}
	}

	node_stack[0].clone()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::Lexer;

	#[test]
	fn postfix_stack_simple() {
		let tok_stream = Lexer::new("1+1");
		let stack = Parser::new(tok_stream).parse();

		assert_eq!(
			stack,
			vec![
				TokenKind::Number(1.),
				TokenKind::Number(1.),
				TokenKind::Plus
			]
		)
	}

	#[test]
	fn postfix_stack_complex() {
		let tok_stream = Lexer::new("A * (B + C * D) + E");
		let stack = Parser::new(tok_stream).parse();

		assert_eq!(
			stack,
			vec![
				TokenKind::Ident("A".into()),
				TokenKind::Ident("B".into()),
				TokenKind::Ident("C".into()),
				TokenKind::Ident("D".into()),
				TokenKind::Multiply,
				TokenKind::Plus,
				TokenKind::Multiply,
				TokenKind::Ident("E".into()),
				TokenKind::Plus,
			]
		)
	}

	#[test]
	fn parse_tree_from_postfix_stack() {
		let tok_stream = Lexer::new("A * (B + C * D) + E");
		let stack = Parser::new(tok_stream).parse();
		let tree = gen_parse_tree(stack);

		assert_eq!(
			Box::new(Node::BinExpr {
				op: TokenKind::Plus,
				lhs: Box::from(Node::BinExpr {
					op: TokenKind::Multiply,
					lhs: Box::from(Node::IdentLiteral("A".into())),
					rhs: Box::from(Node::BinExpr {
						op: TokenKind::Plus,
						lhs: Box::from(Node::IdentLiteral("B".into())),
						rhs: Box::from(Node::BinExpr {
							op: TokenKind::Multiply,
							lhs: Box::from(Node::IdentLiteral("C".into())),
							rhs: Box::from(Node::IdentLiteral("D".into())),
						}),
					}),
				}),
				rhs: Box::from(Node::IdentLiteral("E".into())),
			}),
			tree
		);
	}
}
