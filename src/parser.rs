use crate::lexer::{Lexer, Token, TokenKind};

use std::cmp::Ordering;

enum Association {
	LTR,
	RTL,
	None,
}

impl TokenKind {
	fn get_associativity(&self) -> Association {
		match self {
			TokenKind::LParen | TokenKind::RParen => Association::LTR,
			TokenKind::Multiply | TokenKind::Divide | TokenKind::Mod => {
				Association::LTR
			}
			TokenKind::Plus | TokenKind::Minus => Association::LTR,
			TokenKind::Greater
			| TokenKind::GreaterEq
			| TokenKind::Less
			| TokenKind::LessEq => Association::LTR,
			TokenKind::Equate => Association::LTR,
			TokenKind::BitAnd => Association::LTR,
			TokenKind::Xor => Association::LTR,
			TokenKind::BitOr => Association::LTR,
			TokenKind::And => Association::LTR,
			TokenKind::Or => Association::LTR,
			TokenKind::Assign => Association::RTL,
			_ => Association::None,
		}
	}

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

	fn is_un_op(&self) -> bool {
		matches!(self, TokenKind::Minus | TokenKind::Not | TokenKind::BitNot)
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
	UnaryExpr {
		op: TokenKind,
		rhs: Box<Node>,
	},
	NumberLiteral(f64),
	IdentLiteral(String),
	StringLiteral(String),
}

impl Node {
	fn is_unary_op_ready_node(&self) -> bool {
		matches!(self, Self::UnaryExpr { op: _, rhs: _ } | Self::NumberLiteral(_) | Self::IdentLiteral(_))
	}
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
				match token.kind.get_precedence().cmp(&last.get_precedence()) {
					Ordering::Greater => self.op_stack.push(token.kind),
					Ordering::Equal => match token.kind.get_associativity() {
						Association::LTR => {
							out_stack.push(self.op_stack.pop().unwrap());
							self.op_stack.push(token.kind);
						}
						Association::RTL => {
							self.op_stack.push(token.kind);
						}
						_ => panic!(
							"Unexpected associativity check on non-operator \
							 token!"
						),
					},
					Ordering::Less => {
						out_stack.push(self.op_stack.pop().unwrap());
						self.precedence_based_pushpop(out_stack, token);
					}
				}
			} else {
				self.op_stack.push(token.kind);
			}
		} else {
			self.op_stack.push(token.kind);
		}
	}
}

pub fn gen_parse_tree(out_stack: Vec<TokenKind>) -> Node {
	let mut node_stack = Vec::new();

	for token in &out_stack {
		if token.is_un_op()
			&& (node_stack.last().is_none()
				|| node_stack
					.iter()
					.all(|node: &Node| node.is_unary_op_ready_node()))
		{
			let rhs = Box::new(node_stack.pop().unwrap());
			node_stack.push(Node::UnaryExpr {
				op: token.clone(),
				rhs,
			});
		} else if token.is_op() {
			let rhs = Box::new(node_stack.pop().unwrap());
			let lhs = Box::new(node_stack.pop().unwrap());

			node_stack.push(Node::BinExpr {
				op: token.clone(),
				lhs,
				rhs,
			})
		} else {
			match token.clone() {
				TokenKind::Ident(i) => node_stack.push(Node::IdentLiteral(i)),
				TokenKind::Number(num) => {
					node_stack.push(Node::NumberLiteral(num))
				}
				TokenKind::String(s) => node_stack.push(Node::StringLiteral(s)),
				_ => panic!("Unrecognized non-op token!"),
			}
		}
	}

	node_stack[0].clone()
}

#[cfg(test)]
mod test {
	use super::{gen_parse_tree, Node::*, Parser, TokenKind::*};
	use crate::Lexer;

	#[test]
	fn postfix_stack_simple() {
		let tok_stream = Lexer::new("1+1");
		let stack = Parser::new(tok_stream).parse();

		assert_eq!(stack, vec![Number(1.), Number(1.), Plus])
	}

	#[test]
	fn postfix_stack_complex() {
		let tok_stream = Lexer::new("A * (B + C * D) + E");
		let stack = Parser::new(tok_stream).parse();

		assert_eq!(
			stack,
			vec![
				Ident("A".into()),
				Ident("B".into()),
				Ident("C".into()),
				Ident("D".into()),
				Multiply,
				Plus,
				Multiply,
				Ident("E".into()),
				Plus,
			]
		)
	}

	#[test]
	fn parse_tree_from_postfix_stack() {
		let tok_stream = Lexer::new("A * (B + C * D) + E");
		let stack = Parser::new(tok_stream).parse();
		let tree = gen_parse_tree(stack);

		assert_eq!(
			BinExpr {
				op: Plus,
				lhs: Box::new(BinExpr {
					op: Multiply,
					lhs: Box::new(IdentLiteral("A".into())),
					rhs: Box::new(BinExpr {
						op: Plus,
						lhs: Box::new(IdentLiteral("B".into())),
						rhs: Box::new(BinExpr {
							op: Multiply,
							lhs: Box::new(IdentLiteral("C".into())),
							rhs: Box::new(IdentLiteral("D".into())),
						}),
					}),
				}),
				rhs: Box::new(IdentLiteral("E".into())),
			},
			tree
		);
	}

	#[test]
	fn parse_tree_from_postfix_stack_complex() {
		let tok_stream = Lexer::new("B * C + A / Z - Y");
		let stack = Parser::new(tok_stream).parse();
		let tree = gen_parse_tree(stack);

		assert_eq!(
			tree,
			BinExpr {
				op: Minus,
				lhs: Box::new(BinExpr {
					op: Plus,
					lhs: Box::new(BinExpr {
						op: Multiply,
						lhs: Box::new(IdentLiteral("B".into())),
						rhs: Box::new(IdentLiteral("C".into())),
					}),
					rhs: Box::new(BinExpr {
						op: Divide,
						lhs: Box::new(IdentLiteral("A".into())),
						rhs: Box::new(IdentLiteral("Z".into())),
					}),
				}),
				rhs: Box::new(IdentLiteral("Y".into())),
			}
		);
	}
}
