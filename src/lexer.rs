mod cursor;
mod token;

use cursor::Cursor;

use token::LiteralKind;
use token::Token;
use token::TokenKind;
use token::TokenSpan;

pub struct Lexer<'a> {
	cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			cursor: Cursor::new(input),
		}
	}

	fn skip_whitespaces(&mut self) {
		while let Some(ch) = self.cursor.peek() {
			if char::is_whitespace(*ch) {
				self.cursor.next();
			} else {
				break;
			}
		}
	}

	fn skip_comments(&mut self) {
		loop {
			let mut look = self.cursor.clone();
			if look.next() != Some('/') && look.next() != Some('/') {
				break;
			}

			self.cursor.next();
			self.cursor.next();

			while let Some(&ch) = self.cursor.peek() {
				if ch == '\n' {
					break;
				}

				self.cursor.next();
			}
		}
	}

	fn lex_integer(&mut self) -> Result<Token, LexerError> {
		let start = self.cursor.pos();

		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() {
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Token {
			kind: TokenKind::Literal(LiteralKind::Int),
			span: TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		})
	}

	fn lex_float(&mut self) -> Result<Token, LexerError> {
		let start = self.cursor.pos();

		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() || ch == '.' {
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Token {
			kind: TokenKind::Literal(LiteralKind::Float),
			span: TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		})
	}

	fn validate_ident(&self, ident: &str) -> Option<LexerError> {
		let mut chars = ident.chars();

		match chars.next() {
			Some(ch) if ch == '_' || ch.is_alphanumeric() => {}
			_ => return Some(LexerError::InvalidIdent),
		}

		for ch in ident.chars() {
			if !(ch.is_alphanumeric() || ch == '_') {
				return Some(LexerError::InvalidIdent);
			}
		}

		None
	}

	fn match_keyword(&self, ident: &str, span: TokenSpan) -> Result<Token, LexerError> {
		match ident {
			"var" => Ok(Token {
				kind: TokenKind::KeywordVar,
				span,
			}),
			_ => {
				if let Some(e) = self.validate_ident(ident) {
					return Err(e);
				}

				Ok(Token {
					kind: TokenKind::Ident,
					span,
				})
			}
		}
	}

	fn lex_ident(&mut self) -> Result<Token, LexerError> {
		let start = self.cursor.pos();
		let mut val = String::new();

		while let Some(&ch) = self.cursor.peek() {
			if ch.is_alphanumeric() || ch == '_' {
				val.push(ch);
				self.cursor.next();
			} else {
				break;
			}
		}

		self.match_keyword(
			&val,
			TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		)
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token, LexerError>;

	fn next(&mut self) -> Option<Self::Item> {
		self.skip_whitespaces();
		self.skip_comments();

		let start = self.cursor.pos();
		let kind = match self.cursor.peek()? {
			'0'..='9' => return Some(self.lex_float()),
			'=' => {
				self.cursor.next();
				TokenKind::Assign
			}
			';' => {
				self.cursor.next();
				TokenKind::Semicolon
			}
			_ => return Some(self.lex_ident()),
		};

		Some(Ok(Token {
			kind,
			span: TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		}))
	}
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
	InvalidIdent,
}

#[cfg(test)]
mod tests {
	use crate::lexer::token::LiteralKind;

	use super::*;

	#[test]
	fn lexer_skip_whitespace() {
		let mut l = Lexer::new("     l");
		l.skip_whitespaces();
		assert_eq!(l.cursor.peek(), Some(&'l'));
		assert_eq!(l.cursor.pos(), 5);
	}

	#[test]
	fn lexer_skip_comments() {
		let mut l = Lexer::new("// \n");
		l.skip_comments();
		assert_eq!(l.cursor.peek(), Some(&'\n'));
		assert_eq!(l.cursor.pos(), 3);
	}

	#[test]
	fn lexer_skip_comments_without_new_line() {
		let mut l = Lexer::new("//  ");
		l.skip_comments();
		assert_eq!(l.cursor.peek(), None);
		assert_eq!(l.cursor.pos(), 4);
	}

	#[test]
	fn lexer_lex_integer() {
		let mut l = Lexer::new("69");
		assert_eq!(
			l.lex_integer().unwrap(),
			Token {
				kind: TokenKind::Literal(LiteralKind::Int),
				span: TokenSpan { start: 0, end: 2 }
			}
		);
	}

	#[test]
	fn lexer_lex_float() {
		let mut l = Lexer::new("69.9");
		assert_eq!(
			l.lex_float().unwrap(),
			Token {
				kind: TokenKind::Literal(LiteralKind::Float),
				span: TokenSpan { start: 0, end: 4 }
			}
		);
	}

	#[test]
	fn lexer_lex_float_without_end() {
		let mut l = Lexer::new("0.");
		assert_eq!(
			l.lex_float().unwrap(),
			Token {
				kind: TokenKind::Literal(LiteralKind::Float),
				span: TokenSpan { start: 0, end: 2 },
			}
		);
	}

	#[test]
	fn lexer_lex_ident_var() {
		let mut l = Lexer::new("var");
		assert_eq!(
			l.lex_ident().unwrap(),
			Token {
				kind: TokenKind::KeywordVar,
				span: TokenSpan { start: 0, end: 3 }
			}
		);
	}

	#[test]
	fn lexer_lex_ident_var_with_spaces() {
		let mut l = Lexer::new("var ");
		assert_eq!(
			l.lex_ident().unwrap(),
			Token {
				kind: TokenKind::KeywordVar,
				span: TokenSpan { start: 0, end: 3 }
			}
		);
	}

	#[test]
	fn lexer_lex_ident_identifier() {
		let mut l = Lexer::new("variable_0");
		assert_eq!(
			l.lex_ident().unwrap(),
			Token {
				kind: TokenKind::Ident,
				span: TokenSpan { start: 0, end: 10 }
			},
		)
	}

	#[test]
	fn lexer_lex_ident_identifier_invalid() {
		let mut l = Lexer::new("#invalid$");
		assert_eq!(l.lex_ident().unwrap_err(), LexerError::InvalidIdent);
	}

	#[test]
	fn lexer_iter() {
		let mut vec: Vec<Token> = vec![];
		let l = Lexer::new("var x = 5;");
		for tok in l {
			vec.push(tok.unwrap());
		}

		assert_eq!(
			vec[0],
			Token {
				kind: TokenKind::KeywordVar,
				span: TokenSpan { start: 0, end: 3 }
			}
		);

		assert_eq!(
			vec[1],
			Token {
				kind: TokenKind::Ident,
				span: TokenSpan { start: 4, end: 5 }
			}
		);

		assert_eq!(
			vec[2],
			Token {
				kind: TokenKind::Assign,
				span: TokenSpan { start: 6, end: 7 }
			}
		);

		assert_eq!(
			vec[3],
			Token {
				kind: TokenKind::Literal(LiteralKind::Float),
				span: TokenSpan { start: 8, end: 9 },
			}
		);

		assert_eq!(
			vec[4],
			Token {
				kind: TokenKind::Semicolon,
				span: TokenSpan { start: 9, end: 10 }
			}
		);
	}
}
