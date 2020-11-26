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
			TokenKind::LParen
			| TokenKind::RParen
			| TokenKind::Multiply
			| TokenKind::Divide
			| TokenKind::Mod
			| TokenKind::Plus
			| TokenKind::Minus
			| TokenKind::Greater
			| TokenKind::GreaterEq
			| TokenKind::Less
			| TokenKind::LessEq
			| TokenKind::Equate
			| TokenKind::BitAnd
			| TokenKind::Xor
			| TokenKind::BitOr
			| TokenKind::And
			| TokenKind::Or => Association::LTR,
			TokenKind::Assign | TokenKind::Not | TokenKind::BitNot => {
				Association::RTL
			}
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
		matches!(self, TokenKind::Not | TokenKind::BitNot)
	}
}

pub struct Parser<'a> {
	tokens: Lexer<'a>,
	op_stack: Vec<TokenKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
	// Expressions
	BinExpr {
		op: TokenKind,
		lhs: Box<Node>,
		rhs: Box<Node>,
	},
	UnaryExpr {
		op: TokenKind,
		rhs: Box<Node>,
	},

	// Statements
	Assign {
		// name MUST be an IdentLiteral
		name: String,
		value: Box<Node>,
	},

	// Blocks
	// Block {
	// 	stmts: Vec<Box<Node>>,
	// 	state: HashMap<String, Node>,
	// 	parent: Option<Box<Node>>,
	// },
	// BlockOpen,
	// BlockClose,

	// Literals
	NumberLiteral(f64),
	Ident(String),
	BooleanLiteral(bool),
	StringLiteral(String),
}

impl Node {
	fn is_unary_op_ready_node(&self) -> bool {
		matches!(
			self, Self::UnaryExpr { op: _, rhs: _ }
				| Self::BooleanLiteral(_)
				| Self::Ident(_)
		)
	}

	fn is_stmt(&self) -> bool {
		matches!(self, Self::Assign { name: _, value: _ })
	}
}

impl<'a> Parser<'a> {
	pub fn new(tokens: Lexer<'a>) -> Parser<'a> {
		Self {
			tokens,
			op_stack: Vec::new(),
		}
	}

	pub fn parse(&mut self) -> Vec<Vec<TokenKind>> {
		let mut out_stack = Vec::new();
		let mut fin_stack = Vec::new();

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
			} else if token.kind == TokenKind::Delimeter {
				// TODO: if not func
				for tok in self.op_stack.clone().into_iter().rev() {
					out_stack.push(tok);
				}
				fin_stack.push(out_stack);

				self.op_stack = Vec::new();
				out_stack = Vec::new();
			} else {
				out_stack.push(token.kind);
			}
		}
		for tok in self.op_stack.clone().into_iter().rev() {
			out_stack.push(tok);
		}
		if !out_stack.is_empty() {
			fin_stack.push(out_stack);
		}
		// TODO: error handling: op stack should be empty (specifically, it
		// should be parenthesis-free)
		fin_stack
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
							 token: {:?}",
							token
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

pub fn gen_parse_tree(out_stack: Vec<Vec<TokenKind>>) -> Vec<Node> {
	let mut node_stack = Vec::new();
	let mut stmt_stack = Vec::new();

	for stmt in &out_stack {
		for token in stmt {
			if token.is_un_op()
				|| (token == &TokenKind::Minus
					&& (node_stack.last().is_none()
						|| node_stack
							.iter()
							.all(|node: &Node| node.is_unary_op_ready_node())))
			{
				let rhs = Box::new(node_stack.pop().unwrap());
				node_stack.push(Node::UnaryExpr {
					op: token.clone(),
					rhs,
				});
			} else if token.is_op() {
				let rhs = Box::new(node_stack.pop().unwrap());
				let lhs = node_stack.pop().unwrap();

				if token == &TokenKind::Assign {
					if let Node::Ident(name) = lhs {
						node_stack.push(Node::Assign { name, value: rhs })
					} else {
						panic!(
							"Cannot assign non-ident-literal to a value: {:?}",
							lhs
						);
					}
				} else {
					node_stack.push(Node::BinExpr {
						op: token.clone(),
						lhs: Box::new(lhs),
						rhs,
					})
				}
			} else {
				match token {
					TokenKind::Ident(i) => {
						node_stack.push(Node::Ident(i.clone()))
					}
					TokenKind::Number(num) => {
						node_stack.push(Node::NumberLiteral(*num))
					}
					TokenKind::String(s) => {
						node_stack.push(Node::StringLiteral(s.clone()))
					}
					TokenKind::True => {
						node_stack.push(Node::BooleanLiteral(true))
					}
					TokenKind::False => {
						node_stack.push(Node::BooleanLiteral(false))
					}
					// TokenKind::LBrace => node_stack.push(Node::BlockOpen),
					// TokenKind::RBrace => node_stack.push(Node::BlockClose),
					_ => panic!("Unrecognized non-op token: {:#?}", token),
				}
			}
		}

		if let Some(node) = node_stack.get(0) {
			if node.is_stmt() {
				stmt_stack.push(node_stack[0].clone());
			} else {
				panic!("Encountered non-statement node: {:?}", node_stack[0]);
			}
		}

		node_stack = Vec::new();
	}

	stmt_stack
}

#[cfg(test)]
mod test {
	use super::{gen_parse_tree, Node, Parser, TokenKind};
	use crate::Lexer;
	use pretty_assertions::assert_eq;

	#[test]
	fn postfix_stack_simple() {
		let tok_stream = Lexer::new("1+1");
		let stack = Parser::new(tok_stream).parse();

		assert_eq!(
			stack,
			vec![vec![
				TokenKind::Number(1.),
				TokenKind::Number(1.),
				TokenKind::Plus
			]]
		)
	}

	#[test]
	fn postfix_stack_complex() {
		let tok_stream = Lexer::new("A * (B + C * D) + E");
		let stack = Parser::new(tok_stream).parse();

		assert_eq!(
			stack,
			vec![vec![
				TokenKind::Ident("A".into()),
				TokenKind::Ident("B".into()),
				TokenKind::Ident("C".into()),
				TokenKind::Ident("D".into()),
				TokenKind::Multiply,
				TokenKind::Plus,
				TokenKind::Multiply,
				TokenKind::Ident("E".into()),
				TokenKind::Plus,
			]]
		)
	}

	#[test]
	fn gen_complex_parse_tree() {
		let tok_stream =
			Lexer::new("z = 1 - 1 * 7 - 4 / 3; l = -z + 7 - 2 + 1 / 3");
		let stack = Parser::new(tok_stream).parse();
		let stmts = gen_parse_tree(stack);

		assert_eq!(
			stmts,
			vec![
				Node::Assign {
					name: "z".into(),
					value: Box::new(Node::BinExpr {
						op: TokenKind::Minus,
						lhs: Box::new(Node::BinExpr {
							op: TokenKind::Minus,
							lhs: Box::new(Node::NumberLiteral(1.0)),
							rhs: Box::new(Node::BinExpr {
								op: TokenKind::Multiply,
								lhs: Box::new(Node::NumberLiteral(1.0)),
								rhs: Box::new(Node::NumberLiteral(7.0)),
							}),
						}),
						rhs: Box::new(Node::BinExpr {
							op: TokenKind::Divide,
							lhs: Box::new(Node::NumberLiteral(4.0)),
							rhs: Box::new(Node::NumberLiteral(3.0)),
						}),
					}),
				},
				Node::Assign {
					name: "l".into(),
					value: Box::new(Node::BinExpr {
						op: TokenKind::Plus,
						lhs: Box::new(Node::BinExpr {
							op: TokenKind::Minus,
							lhs: Box::new(Node::BinExpr {
								op: TokenKind::Plus,
								lhs: Box::new(Node::UnaryExpr {
									op: TokenKind::Minus,
									rhs: Box::new(Node::Ident("z".into())),
								}),
								rhs: Box::new(Node::NumberLiteral(7.0)),
							}),
							rhs: Box::new(Node::NumberLiteral(2.0)),
						}),
						rhs: Box::new(Node::BinExpr {
							op: TokenKind::Divide,
							lhs: Box::new(Node::NumberLiteral(1.0)),
							rhs: Box::new(Node::NumberLiteral(3.0)),
						}),
					}),
				},
			]
		)
	}
}
