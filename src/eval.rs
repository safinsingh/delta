use crate::{lexer::TokenKind, parser::Node};
use std::{fmt, ops};

pub enum NodeResult {
	Numeric(f64),
	Boolean(bool),
	String(String),
	Undefined,
}

impl fmt::Debug for NodeResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Numeric(n) => write!(f, "{}", n),
			Self::Boolean(n) => write!(f, "{}", n),
			Self::String(n) => write!(f, "{}", n),
			Self::Undefined => write!(f, "UNDEFINED"),
		}
	}
}

// https://docs.rs/stdext/0.2.1/src/stdext/macros.rs.html#61-72
macro_rules! function_name {
	() => {{
		// Okay, this is ugly, I get it. However, this is the best we can get on
		// a stable rust.
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str { std::any::type_name::<T>() }
		let name = type_name_of(f);
		// `3` is the length of the `::f`.
		&name[..name.len() - 3]
		}};
}

macro_rules! eval_panic {
	($lhs:ident, $rhs:ident) => {
		panic!(
			"Cannot apply operation {} to {:?} and {:?}",
			function_name!(),
			$lhs,
			$rhs
			)
	};
}

macro_rules! unary_eval_panic {
	($rhs:ident) => {
		panic!("Cannot apply operation {} to {:?}", function_name!(), $rhs);
	};
}

impl ops::Add for NodeResult {
	type Output = NodeResult;

	fn add(self, rhs: NodeResult) -> Self::Output {
		match self {
			NodeResult::Numeric(lhs) => match rhs {
				NodeResult::Numeric(rhs_ex) => {
					NodeResult::Numeric(lhs + rhs_ex)
				}
				NodeResult::String(rhs_ex) => {
					NodeResult::String(format!("{}{}", lhs, rhs_ex))
				}
				_ => eval_panic!(lhs, rhs),
			},
			NodeResult::String(lhs) => match rhs {
				NodeResult::Numeric(rhs_ex) => {
					NodeResult::String(format!("{}{}", lhs, rhs_ex))
				}
				NodeResult::Boolean(rhs_ex) => {
					NodeResult::String(format!("{}{}", lhs, rhs_ex))
				}
				NodeResult::String(rhs_ex) => {
					NodeResult::String(format!("{}{}", lhs, rhs_ex))
				}
				_ => eval_panic!(lhs, rhs),
			},
			NodeResult::Boolean(lhs) => match rhs {
				NodeResult::String(rhs_ex) => {
					NodeResult::String(format!("{}{}", lhs, rhs_ex))
				}
				_ => eval_panic!(lhs, rhs),
			},
			_ => eval_panic!(self, rhs),
		}
	}
}

impl ops::Sub for NodeResult {
	type Output = NodeResult;

	fn sub(self, rhs: NodeResult) -> Self::Output {
		match self {
			NodeResult::Numeric(lhs) => match rhs {
				NodeResult::Numeric(rhs_ex) => {
					NodeResult::Numeric(lhs - rhs_ex)
				}
				_ => eval_panic!(lhs, rhs),
			},
			_ => eval_panic!(self, rhs),
		}
	}
}

impl ops::Not for NodeResult {
	type Output = NodeResult;

	fn not(self) -> Self::Output {
		match self {
			NodeResult::Boolean(b) => NodeResult::Boolean(!b),
			_ => unary_eval_panic!(self),
		}
	}
}

impl Node {
	pub fn eval(&self) -> NodeResult {
		match self {
			Self::BinExpr { op, lhs, rhs } => match op {
				TokenKind::Plus => lhs.eval() + rhs.eval(),
				TokenKind::Minus => lhs.eval() - rhs.eval(),
				// TokenKind::Multiply => return lhs.eval() * rhs.eval(),
				// TokenKind::Divide => return lhs.eval() / rhs.eval(),
				// TokenKind::Mod => return lhs.eval() % rhs.eval(),
				// TokenKind::Greater => return lhs.eval() > rhs.eval(),
				// TokenKind::GreaterEq => return lhs.eval() >= rhs.eval(),
				// TokenKind::Less => return lhs.eval() < rhs.eval(),
				// TokenKind::LessEq => return lhs.eval() <= rhs.eval(),
				// TokenKind::Equate => return lhs.eval() == rhs.eval(),
				// TokenKind::BitAnd => return lhs.eval() & rhs.eval(),
				// TokenKind::Xor => return lhs.eval() ^ rhs.eval(),
				// TokenKind::BitOr => return lhs.eval() | rhs.eval(),
				// TokenKind::And => return lhs.eval() && rhs.eval(),
				// TokenKind::Or => return lhs.eval() || rhs.eval(),
				_ => todo!("Unrecognized binary op!"),
			},
			Self::UnaryExpr { op, rhs } => match op {
				TokenKind::Not => !rhs.eval(),
				_ => todo!("Unrecognized unary op!"),
			},
			Self::Assign { name: _, value } => value.eval(),
			Self::NumberLiteral(num) => NodeResult::Numeric(*num),
			Self::StringLiteral(s) => NodeResult::String(s.clone()),
			Self::BooleanLiteral(b) => NodeResult::Boolean(*b),
			_ => todo!("Unrecognized node!"),
		}
	}
}
