mod cursor;
mod token;

use std::collections::HashMap;

use cursor::Cursor;

use crate::lexer::token::{Token, TokenKind};

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
		let mut val = String::new();
		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() {
				val.push(ch);
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Token {
			kind: TokenKind::LiteralInt(val.parse().unwrap()),
		})
	}

	fn lex_float(&mut self) -> Result<Token, LexerError> {
		let mut val = String::new();
		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() || ch == '.' {
				val.push(ch);
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Token {
			kind: TokenKind::LiteralFloat(val.parse::<f64>().unwrap()),
		})
	}

	fn match_keyword(&mut self, indent: &str) -> Token {
		match indent {
			"var" => Token {
				kind: TokenKind::KeywordVar,
			},
			_ => Token {
				kind: TokenKind::Indent(indent.to_string()),
			},
		}
	}

	fn lex_indent(&mut self) -> Result<Token, LexerError> {
		let mut val = String::new();
		while let Some(&ch) = self.cursor.peek() {
			if ch.is_alphanumeric() || ch == '_' {
				val.push(ch);
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(self.match_keyword(&val))
	}
}

/// TODO: Create Lexer Errors
#[derive(Debug)]
enum LexerError {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lexer_skip_whitespace() {
		let mut l = Lexer::new("     l");
		l.skip_whitespaces();
		assert_eq!(l.cursor.peek(), Some(&'l'));
	}

	#[test]
	fn lexer_skip_comments() {
		let mut l = Lexer::new("// \n");
		l.skip_comments();
		assert_eq!(l.cursor.peek(), Some(&'\n'));
	}

	#[test]
	fn lexer_skip_comments_without_new_line() {
		let mut l = Lexer::new("//  ");
		l.skip_comments();
		assert_eq!(l.cursor.peek(), None);
	}

	#[test]
	fn lexer_lex_integer() {
		let mut l = Lexer::new("69");
		assert_eq!(l.lex_integer().unwrap().kind, TokenKind::LiteralInt(69));
	}

	#[test]
	fn lexer_lex_float() {
		let mut l = Lexer::new("69.9");
		assert_eq!(l.lex_float().unwrap().kind, TokenKind::LiteralFloat(69.9));
	}

	#[test]
	fn lexer_lex_float_without_end() {
		let mut l = Lexer::new("0.");
		assert_eq!(l.lex_float().unwrap().kind, TokenKind::LiteralFloat(0.));
	}

	#[test]
	fn lexer_lex_indent_var() {
		let mut l = Lexer::new("var");
		assert_eq!(l.lex_indent().unwrap().kind, TokenKind::KeywordVar);
	}

	#[test]
	fn lexer_lex_indent_var_with_spaces() {
		let mut l = Lexer::new("var ");
		assert_eq!(l.lex_indent().unwrap().kind, TokenKind::KeywordVar);
	}

	#[test]
	fn lexer_lex_indent_indentifier() {
		let mut l = Lexer::new("variable_0");
		assert_eq!(
			l.lex_indent().unwrap().kind,
			TokenKind::Indent("variable_0".to_string()),
		)
	}
}
