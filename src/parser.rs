mod parser_cursor;

use parser_cursor::Expected;
use parser_cursor::ParseError;
use parser_cursor::ParserCursor;

use crate::ast::AstLiteralKind;
use crate::ast::BinaryOp;
use crate::ast::Expr;
use crate::ast::Stmt;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::lexer::token::TokenLiteralKind;

pub struct Parser {
	cursor: ParserCursor,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			cursor: ParserCursor::new(tokens),
		}
	}

	pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
		let mut stmts: Vec<Stmt> = vec![];
		while !self.cursor.at_eof() {
			stmts.push(dbg!(self.parse_stmt()?));
		}

		Ok(stmts)
	}

	fn parse_expr(&mut self) -> Result<Expr, ParseError> {
		dbg!(self.cursor.peek());
		match self.cursor.peek_kind() {
			Some(TokenKind::Literal(_)) => self.parse_literal(),
			Some(TokenKind::Ident) => self.parse_ident(),
			other => Err(ParseError::Unexpected(other.cloned(), Expected::Expr)),
		}
	}

	fn parse_literal(&mut self) -> Result<Expr, ParseError> {
		match self.cursor.next() {
			Some(Token {
				kind: TokenKind::Literal(TokenLiteralKind::Int),
				span: _,
				orig,
			}) => Ok(Expr::Literal(AstLiteralKind::Int(
				orig.parse::<i64>().unwrap(),
			))),
			Some(Token {
				kind: TokenKind::Literal(TokenLiteralKind::Float),
				span: _,
				orig,
			}) => Ok(Expr::Literal(AstLiteralKind::Float(
				orig.parse::<f64>().unwrap(),
			))),
			Some(Token {
				kind,
				span: _,
				orig: _,
			}) => Err(ParseError::Unexpected(
				Some(kind.clone()),
				Expected::Literal,
			)),
			_ => Err(ParseError::Unexpected(None, Expected::Literal)),
		}
	}

	fn expect_ident(&mut self) -> Result<String, ParseError> {
		match self.cursor.next() {
			Some(Token {
				kind: TokenKind::Ident,
				span: _,
				orig,
			}) => Ok(orig),
			_ => Err(ParseError::ExpectedIdent),
		}
	}

	fn parse_ident(&mut self) -> Result<Expr, ParseError> {
		Ok(Expr::Ident(self.expect_ident()?))
	}

	fn parse_assign(&mut self) -> Result<Stmt, ParseError> {
		let left = self.parse_ident()?;
		self.cursor
			.expect(&TokenKind::Assign, Expected::Kind(TokenKind::Assign))?;

		let right = self.parse_literal()?;

		Ok(Stmt::Expr(Expr::Binary {
			op: BinaryOp::Assign,
			left: Box::new(left),
			right: Box::new(right),
		}))
	}

	fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
		match self.cursor.peek_kind() {
			Some(TokenKind::KeywordVar) => self.parse_var(),
			Some(TokenKind::Ident)
				if matches!(self.cursor.peekn_kind(1), Some(TokenKind::Assign)) =>
			{
				let stmt = self.parse_assign()?;
				self.cursor.expect(
					&TokenKind::Semicolon,
					Expected::Kind(TokenKind::Semicolon),
				)?;

				Ok(stmt)
			}
			_ => {
				let expr = self.parse_expr()?;
				self.cursor.expect(
					&TokenKind::Semicolon,
					Expected::Kind(TokenKind::Semicolon),
				)?;
				Ok(Stmt::Expr(expr))
			}
		}
	}

	fn parse_var(&mut self) -> Result<Stmt, ParseError> {
		self.cursor.expect(
			&TokenKind::KeywordVar,
			Expected::Kind(TokenKind::KeywordVar),
		)?;
		let ident = self.expect_ident()?;

		let value = match self.cursor.peek_kind() {
			Some(TokenKind::Semicolon) => None,
			Some(TokenKind::Assign) => {
				self.cursor.next();

				Some(self.parse_expr()?)
			}
			other => {
				return Err(ParseError::Unexpected(
					other.cloned(),
					Expected::Kind(TokenKind::Semicolon),
				));
			}
		};

		self.cursor
			.expect(&TokenKind::Semicolon, Expected::Kind(TokenKind::Semicolon))?;
		Ok(Stmt::Var { ident, value })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::lexer::Lexer;

	fn parse_from_string(source: &str) -> Vec<Stmt> {
		let tokens: Vec<Token> = Lexer::new(source)
			.map(|r| r.expect("lexer error"))
			.collect();

		let mut p = Parser::new(tokens);
		p.parse().expect("parse error")
	}

	#[test]
	fn parse_var_empty_declaration() {
		let ast = parse_from_string("var x;");
		assert!(matches!(&ast[0], Stmt::Var { ident, .. } if ident == "x"))
	}

	#[test]
	fn parse_var_declaration() {
		let ast = parse_from_string("var x = 5;");
		assert!(
			matches!(&ast[0], Stmt::Var { ident, value } if ident == "x" &&
				*value == Some(Expr::Literal(AstLiteralKind::Int(5))))
		)
	}

	#[test]
	fn parse_float_var_declaration() {
		let ast = parse_from_string("var x = 5.5;");
		assert!(
			matches!(&ast[0], Stmt::Var { ident, value } if ident == "x" &&
				*value == Some(Expr::Literal(AstLiteralKind::Float(5.5))))
		)
	}

	#[test]
	fn parse_multiple_var_declaration() {
		let ast = parse_from_string("var x = 5; var y = 6; var z = 7;");
		assert_eq!(ast.len(), 3);
	}

	#[test]
	fn parse_var_stmt() {
		let ast = parse_from_string("var x = 5; x;");
		assert_eq!(ast.len(), 2);
		assert!(matches!(&ast[1], Stmt::Expr(Expr::Ident(ident)) if ident == "x"));
	}

	#[test]
	fn parse_empty_string() {
		let ast = parse_from_string("");
		assert!(ast.is_empty())
	}

	#[test]
	fn parse_assign() {
		let ast = parse_from_string("var x = 5; x = 3;");
		assert!(matches!(
			&ast[1],
			Stmt::Expr(Expr::Binary { op, left, right }) if *op == BinaryOp::Assign &&
			**left == Expr::Ident("x".to_string()) &&
			**right == Expr::Literal(AstLiteralKind::Int(3))
		))
	}
}
