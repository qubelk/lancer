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
			if self.cursor.peek() != Some(&'/') {
				break;
			}

			let mut look = self.cursor.clone();
			if look.next() != Some('/') {
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

	fn lex_number(&mut self) -> Result<Token, LexerError> {
		let start = self.cursor.pos();
		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() {
				self.cursor.next();
			} else {
				break;
			}
		}

		let mut kind = LiteralKind::Int;

		if self.cursor.peek() == Some(&'.') {
			let mut look = self.cursor.clone();
			look.next();
			if look.peek().is_some_and(|ch| ch.is_ascii_digit()) {
				kind = LiteralKind::Float;
				self.cursor.next();
				while let Some(&ch) = self.cursor.peek() {
					if ch.is_ascii_digit() {
						self.cursor.next();
					}
				}
			}
		}
		Ok(Token {
			kind: TokenKind::Literal(kind),
			span: TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		})
	}

	fn match_keyword(&self, ident: &str, span: TokenSpan) -> Token {
		match ident {
			"var" => Token {
				kind: TokenKind::KeywordVar,
				span,
			},
			_ => Token {
				kind: TokenKind::Ident,
				span,
			},
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

		Ok(self.match_keyword(
			&val,
			TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		))
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token, LexerError>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let pos = self.cursor.pos();
			self.skip_whitespaces();
			self.skip_comments();
			if pos == self.cursor.pos() {
				break;
			}
		}

		let start = self.cursor.pos();
		let kind = match self.cursor.peek()? {
			'0'..='9' => return Some(self.lex_number()),
			'=' => {
				self.cursor.next();
				TokenKind::Assign
			}
			';' => {
				self.cursor.next();
				TokenKind::Semicolon
			}
			c if *c == '_' || c.is_alphanumeric() => return Some(self.lex_ident()),
			_ => {
				self.cursor.next();
				return Some(Err(LexerError::UnexpectedChar {}));
			}
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
	UnexpectedChar,
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
			l.lex_number().unwrap(),
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
			l.lex_number().unwrap(),
			Token {
				kind: TokenKind::Literal(LiteralKind::Float),
				span: TokenSpan { start: 0, end: 4 }
			}
		);
	}

	// TODO: Create LexerError for this case.
	#[test]
	fn lexer_lex_float_without_end() {
		let mut l = Lexer::new("0.");
		assert_eq!(
			l.lex_number().unwrap(),
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

	// TODO: Fix lex_ident parsing.
	#[test]
	fn lexer_lex_ident_identifier_invalid() {
		let mut l = Lexer::new("#invalid$");
		assert_eq!(
			l.lex_ident().unwrap(),
			Token {
				kind: TokenKind::Ident,
				span: TokenSpan { start: 0, end: 0 }
			}
		);
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
				kind: TokenKind::Literal(LiteralKind::Int),
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
