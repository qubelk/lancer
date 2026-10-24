mod parser_cursor;

use parser_cursor::Expected;
use parser_cursor::ParseError;
use parser_cursor::ParserCursor;

use crate::ast::AstLiteralKind;
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

	fn parse_stmt() -> Result<Stmt, ParseError> {
		todo!()
	}

	fn parse_var(&mut self) -> Result<Stmt, ParseError> {
		self.cursor.expect(
			&TokenKind::KeywordVar,
			Expected::Kind(TokenKind::KeywordVar),
		)?;
		let ident = self.expect_ident()?;

		self.cursor
			.expect(&TokenKind::Assign, Expected::Kind(TokenKind::Assign))?;
		let value = self.parse_expr()?;
		self.cursor
			.expect(&TokenKind::Semicolon, Expected::Kind(TokenKind::Assign))?;

		Ok(Stmt::Var {
			ident: ident,
			value,
		})
	}
}
