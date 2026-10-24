mod lexer_cursor;
pub mod token;

use lexer_cursor::LexerCursor;

use token::LiteralKind;
use token::Token;
use token::TokenKind;
use token::TokenSpan;

pub struct Lexer<'a> {
	cursor: LexerCursor<'a>,
	eof_emitted: bool,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			cursor: LexerCursor::new(input),
			eof_emitted: false,
		}
	}

	fn skip_whitespaces(&mut self) {
		while let Some(ch) = self.cursor.peek() {
			if ch.is_ascii_whitespace() {
				self.cursor.next();
			} else {
				break;
			}
		}
	}

	fn skip_comments(&mut self) {
		while self.cursor.peek() == Some('/') && self.cursor.peekn(1) == Some('/') {
			self.cursor.next();
			self.cursor.next();
			while let Some(ch) = self.cursor.peek() {
				if ch == '\n' {
					break;
				}

				self.cursor.next();
			}
		}
	}

	fn lex_float(&mut self) -> Result<LiteralKind, LexerError> {
		let dot_position = self.cursor.pos();
		self.cursor.next();

		match self.cursor.peek() {
			Some(ch) if ch.is_ascii_digit() => {
				let kind = LiteralKind::Float;
				self.cursor.next();
				while let Some(ch) = self.cursor.peek() {
					if ch.is_ascii_digit() {
						self.cursor.next();
					} else if ch == '.' {
						let start = self.cursor.pos();
						self.cursor.next();
						return Err(LexerError::UnexpectedChar(
							ch,
							TokenSpan {
								start,
								end: self.cursor.pos(),
							},
						));
					} else {
						break;
					}
				}

				Ok(kind)
			}
			_ => Err(LexerError::UnexpectedChar(
				'.',
				TokenSpan {
					start: dot_position,
					end: dot_position + 1,
				},
			)),
		}
	}

	fn lex_number(&mut self) -> Result<Token, LexerError> {
		let start = self.cursor.pos();
		while let Some(ch) = self.cursor.peek() {
			if ch.is_ascii_digit() {
				self.cursor.next();
			} else {
				break;
			}
		}

		let mut kind = LiteralKind::Int;

		if self.cursor.peek() == Some('.') {
			kind = self.lex_float()?;
		}

		Ok(Token {
			kind: TokenKind::Literal(kind),
			span: TokenSpan {
				start,
				end: self.cursor.pos(),
			},
			orig: self.cursor.slice(start, self.cursor.pos()),
		})
	}

	fn match_keyword(ident: &str, span: TokenSpan) -> Token {
		let kind = match ident {
			"var" => TokenKind::KeywordVar,
			_ => TokenKind::Ident,
		};

		Token {
			kind,
			span,
			orig: ident.to_string(),
		}
	}

	fn lex_ident(&mut self) -> Result<Token, LexerError> {
		let start = self.cursor.pos();

		let first = match self.cursor.peek() {
			Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => ch,
			Some(ch) => {
				let start = self.cursor.pos();
				self.cursor.next();
				return Err(LexerError::UnexpectedChar(
					ch,
					TokenSpan {
						start,
						end: self.cursor.pos(),
					},
				));
			}
			None => {
				return Err(LexerError::UnexpectedChar(
					'\0',
					TokenSpan {
						start,
						end: self.cursor.pos(),
					},
				));
			}
		};

		let mut val = String::new();
		val.push(first);
		self.cursor.next();

		while let Some(ch) = self.cursor.peek() {
			if ch.is_ascii_alphanumeric() || ch == '_' {
				val.push(ch);
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Self::match_keyword(
			&val,
			TokenSpan {
				start,
				end: self.cursor.pos(),
			},
		))
	}

	fn skip_trivia(&mut self) {
		loop {
			let pos = self.cursor.pos();
			self.skip_whitespaces();
			self.skip_comments();
			if pos == self.cursor.pos() {
				break;
			}
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token, LexerError>;

	fn next(&mut self) -> Option<Self::Item> {
		self.skip_trivia();

		if self.eof_emitted {
			return None;
		}

		let start = self.cursor.pos();
		let kind = match self.cursor.peek() {
			Some(ch) => match ch {
				'0'..='9' => return Some(self.lex_number()),
				'=' => {
					self.cursor.next();
					TokenKind::Assign
				}
				';' => {
					self.cursor.next();
					TokenKind::Semicolon
				}
				ch if ch == '_' || ch.is_ascii_alphanumeric() => {
					return Some(self.lex_ident());
				}
				ch => {
					self.cursor.next();
					return Some(Err(LexerError::UnexpectedChar(
						ch,
						TokenSpan {
							start,
							end: self.cursor.pos(),
						},
					)));
				}
			},
			None => {
				self.eof_emitted = true;
				TokenKind::Eof
			}
		};

		Some(Ok(Token {
			kind,
			span: TokenSpan {
				start,
				end: self.cursor.pos(),
			},
			orig: self.cursor.slice(start, self.cursor.pos()),
		}))
	}
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
	UnexpectedChar(char, TokenSpan),
}

#[cfg(test)]
mod tests {
	use crate::lexer::token::LiteralKind;

	use super::*;

	#[test]
	fn lexer_skip_whitespace() {
		let mut l = Lexer::new("     l");
		l.skip_whitespaces();
		assert_eq!(l.cursor.peek(), Some('l'));
		assert_eq!(l.cursor.pos(), 5);
	}

	#[test]
	fn lexer_skip_comments() {
		let mut l = Lexer::new("// \n");
		l.skip_comments();
		assert_eq!(l.cursor.peek(), Some('\n'));
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
	fn lexer_skip_trivia() {
		let mut l = Lexer::new("  //   ");
		l.skip_trivia();
		assert_eq!(l.cursor.peek(), None);
		assert_eq!(l.cursor.pos(), 7);
	}

	#[test]
	fn lexer_skip_trivia_without_trivia() {
		let mut l = Lexer::new("");
		l.skip_trivia();
		assert_eq!(l.cursor.peek(), None);
		assert_eq!(l.cursor.pos(), 0);
	}

	#[test]
	fn lexer_lex_integer() {
		let mut l = Lexer::new("69");
		assert_eq!(
			l.lex_number().unwrap(),
			Token {
				kind: TokenKind::Literal(LiteralKind::Int),
				span: TokenSpan { start: 0, end: 2 },
				orig: "69".to_string(),
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
				span: TokenSpan { start: 0, end: 4 },
				orig: "69.9".to_string(),
			}
		);
	}

	#[test]
	fn lexer_lex_float_without_end() {
		let mut l = Lexer::new("0.");
		assert_eq!(
			l.lex_number(),
			Err(LexerError::UnexpectedChar(
				'.',
				TokenSpan { start: 1, end: 2 },
			))
		);
	}

	#[test]
	fn lexer_lex_float_with_multiple_dots() {
		let mut l = Lexer::new("1.2.3");
		assert_eq!(
			l.lex_number(),
			Err(LexerError::UnexpectedChar(
				'.',
				TokenSpan { start: 3, end: 4 }
			))
		);
	}

	#[test]
	fn lexer_lex_ident_var() {
		let mut l = Lexer::new("var");
		assert_eq!(
			l.lex_ident().unwrap(),
			Token {
				kind: TokenKind::KeywordVar,
				span: TokenSpan { start: 0, end: 3 },
				orig: "var".to_string(),
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
				span: TokenSpan { start: 0, end: 3 },
				orig: "var".to_string(),
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
				span: TokenSpan { start: 0, end: 10 },
				orig: "variable_0".to_string(),
			},
		)
	}

	#[test]
	fn lexer_lex_ident_identifier_invalid() {
		let mut l = Lexer::new("#invalid$");
		assert_eq!(
			l.lex_ident(),
			Err(LexerError::UnexpectedChar(
				'#',
				TokenSpan { start: 0, end: 1 }
			))
		);
	}

	#[test]
	fn lexer_lex_ident_start_from_number() {
		let mut l = Lexer::new("1_ident");
		assert_eq!(
			l.lex_ident(),
			Err(LexerError::UnexpectedChar(
				'1',
				TokenSpan { start: 0, end: 1 }
			))
		)
	}

	#[test]
	fn lexer_lex_ident_empty() {
		let mut l = Lexer::new("");
		assert_eq!(
			l.lex_ident(),
			Err(LexerError::UnexpectedChar(
				'\0',
				TokenSpan { start: 0, end: 0 }
			))
		)
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
				span: TokenSpan { start: 0, end: 3 },
				orig: "var".to_string(),
			}
		);

		assert_eq!(
			vec[1],
			Token {
				kind: TokenKind::Ident,
				span: TokenSpan { start: 4, end: 5 },
				orig: "x".to_string(),
			}
		);

		assert_eq!(
			vec[2],
			Token {
				kind: TokenKind::Assign,
				span: TokenSpan { start: 6, end: 7 },
				orig: "=".to_string(),
			}
		);

		assert_eq!(
			vec[3],
			Token {
				kind: TokenKind::Literal(LiteralKind::Int),
				span: TokenSpan { start: 8, end: 9 },
				orig: "5".to_string(),
			}
		);

		assert_eq!(
			vec[4],
			Token {
				kind: TokenKind::Semicolon,
				span: TokenSpan { start: 9, end: 10 },
				orig: ";".to_string(),
			}
		);

		assert_eq!(
			vec[5],
			Token {
				kind: TokenKind::Eof,
				span: TokenSpan { start: 10, end: 10 },
				orig: "".to_string(),
			}
		)
	}

	#[test]
	fn lexer_iter_with_float() {
		let mut vec: Vec<Token> = vec![];
		let l = Lexer::new("var x = 5.5;");
		for tok in l {
			vec.push(tok.unwrap());
		}

		assert_eq!(
			vec[0],
			Token {
				kind: TokenKind::KeywordVar,
				span: TokenSpan { start: 0, end: 3 },
				orig: "var".to_string(),
			}
		);

		assert_eq!(
			vec[1],
			Token {
				kind: TokenKind::Ident,
				span: TokenSpan { start: 4, end: 5 },
				orig: "x".to_string(),
			}
		);

		assert_eq!(
			vec[2],
			Token {
				kind: TokenKind::Assign,
				span: TokenSpan { start: 6, end: 7 },
				orig: "=".to_string(),
			}
		);

		assert_eq!(
			vec[3],
			Token {
				kind: TokenKind::Literal(LiteralKind::Float),
				span: TokenSpan { start: 8, end: 11 },
				orig: "5.5".to_string(),
			}
		);

		assert_eq!(
			vec[4],
			Token {
				kind: TokenKind::Semicolon,
				span: TokenSpan { start: 11, end: 12 },
				orig: ";".to_string(),
			}
		);

		assert_eq!(
			vec[5],
			Token {
				kind: TokenKind::Eof,
				span: TokenSpan { start: 12, end: 12 },
				orig: "".to_string(),
			}
		)
	}

	#[test]
	fn lexer_iter_with_invalid_ident() {
		let mut vec: Vec<Result<Token, LexerError>> = vec![];
		let l = Lexer::new("var $ = 5;");
		for tok in l {
			vec.push(tok);
		}

		assert_eq!(
			vec[0],
			Ok(Token {
				kind: TokenKind::KeywordVar,
				span: TokenSpan { start: 0, end: 3 },
				orig: "var".to_string(),
			})
		);

		assert_eq!(
			vec[1],
			Err(LexerError::UnexpectedChar(
				'$',
				TokenSpan { start: 4, end: 5 }
			))
		);

		assert_eq!(
			vec[2],
			Ok(Token {
				kind: TokenKind::Assign,
				span: TokenSpan { start: 6, end: 7 },
				orig: "=".to_string(),
			})
		);

		assert_eq!(
			vec[3],
			Ok(Token {
				kind: TokenKind::Literal(LiteralKind::Int),
				span: TokenSpan { start: 8, end: 9 },
				orig: "5".to_string(),
			})
		);

		assert_eq!(
			vec[4],
			Ok(Token {
				kind: TokenKind::Semicolon,
				span: TokenSpan { start: 9, end: 10 },
				orig: ";".to_string(),
			})
		);

		assert_eq!(
			vec[5],
			Ok(Token {
				kind: TokenKind::Eof,
				span: TokenSpan { start: 10, end: 10 },
				orig: "".to_string(),
			})
		)
	}
}
