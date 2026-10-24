mod parser_cursor;

use parser_cursor::ParseError;
use parser_cursor::ParserCursor;

use crate::ast;
use crate::ast::Expr;
use crate::ast::Stmt;
use crate::lexer::token;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

pub struct Parser {
	cursor: ParserCursor,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			cursor: ParserCursor::new(tokens),
		}
	}

	pub fn parse() -> Result<Vec<Stmt>, ParseError> {
		todo!();
	}

	fn parse_expr(&mut self) -> Result<Expr, ParseError> {
		match self.cursor.peek_kind() {
			Some(TokenKind::Literal(_)) => self.parse_literal(),
			Some(TokenKind::Ident) => self.parse_ident(),
			_ => todo!(),
		}
	}

	fn parse_literal(&mut self) -> Result<Expr, ParseError> {
		match self.cursor.peek() {
			Some(Token {
				kind: TokenKind::Literal(token::LiteralKind::Int),
				span: _,
				orig,
			}) => Ok(Expr::Literal(ast::LiteralKind::Int(
				orig.parse::<i64>().unwrap(),
			))),
			Some(Token {
				kind: TokenKind::Literal(token::LiteralKind::Float),
				span: _,
				orig,
			}) => Ok(Expr::Literal(ast::LiteralKind::Float(
				orig.parse::<f64>().unwrap(),
			))),
			Some(Token {
				kind,
				span: _,
				orig: _,
			}) => Err(ParseError::Unexpected(
				Some(kind.clone()),
				TokenKind::Literal(token::LiteralKind::Any),
			)),
			_ => Err(ParseError::Unexpected(
				None,
				TokenKind::Literal(token::LiteralKind::Any),
			)),
		}
	}

	fn parse_ident(&mut self) -> Result<Expr, ParseError> {
		match self.cursor.peek() {
			Some(Token {
				kind: TokenKind::Ident,
				span: _,
				orig,
			}) => Ok(Expr::Ident(orig.to_string())),
			_ => Err(ParseError::ExpectedIdent),
		}
	}

	fn parse_stmt() -> Result<Stmt, ParseError> {
		todo!()
	}

	fn parse_var(&mut self) -> Result<Stmt, ParseError> {
		self.cursor.expect(&TokenKind::KeywordVar)?;
		let ident = match self.cursor.next() {
			Some(Token {
				kind: TokenKind::Ident,
				span: _,
				orig,
			}) => orig,
			_ => return Err(ParseError::ExpectedIdent),
		};

		self.cursor.expect(&TokenKind::Assign)?;
		let value = self.parse_expr()?;
		self.cursor.expect(&TokenKind::Semicolon)?;

		Ok(Stmt::Var {
			ident: ident,
			value,
		})
	}
}
