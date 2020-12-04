#![allow(dead_code)]

use crate::lexer::{Token, TokenKind};

#[derive(PartialEq)]
pub(crate) enum Association {
	LTR,
	RTL,
	None,
}

impl Token {
	pub(crate) fn get_associativity(&self) -> Association {
		match self.kind {
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
			| TokenKind::Eq
			| TokenKind::NotEq
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

	pub(crate) fn get_precedence(&self) -> u8 {
		match self.kind {
			TokenKind::LParen | TokenKind::RParen => 12,
			TokenKind::Not | TokenKind::BitNot => 11,
			TokenKind::Multiply | TokenKind::Divide | TokenKind::Mod => 10,
			TokenKind::Plus | TokenKind::Minus => 9,
			TokenKind::Greater
			| TokenKind::GreaterEq
			| TokenKind::Less
			| TokenKind::LessEq => 8,
			TokenKind::Eq | TokenKind::NotEq => 7,
			TokenKind::BitAnd => 6,
			TokenKind::Xor => 5,
			TokenKind::BitOr => 4,
			TokenKind::And => 3,
			TokenKind::Or => 2,
			TokenKind::Assign => 1,
			_ => 0,
		}
	}

	pub(crate) fn is_op(&self) -> bool { self.get_precedence() > 0 }

	pub(crate) fn is_un_op(&self) -> bool {
		matches!(self.kind, TokenKind::Not | TokenKind::BitNot)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
	// Expressions
	BinExpr {
		op: Token,
		lhs: Box<Node>,
		rhs: Box<Node>,
	},
	UnaryExpr {
		op: Token,
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
}
