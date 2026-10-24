mod parser_cursor;

use parser_cursor::ParseError;
use parser_cursor::ParserCursor;

use crate::ast::Expr;
use crate::ast::Stmt;
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

	fn parse_stmt() -> Result<Stmt, ParseError> {
		todo!()
	}

	fn parse_expr(&mut self) -> Result<Expr, ParseError> {
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
