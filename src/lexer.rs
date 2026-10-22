mod cursor;
mod token;

use cursor::Cursor;

use token::LiteralKind;
use token::Token;
use token::TokenKind;

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
		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() {
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Token {
			kind: TokenKind::Literal(LiteralKind::Int),
		})
	}

	fn lex_float(&mut self) -> Result<Token, LexerError> {
		while let Some(&ch) = self.cursor.peek() {
			if ch.is_ascii_digit() || ch == '.' {
				self.cursor.next();
			} else {
				break;
			}
		}

		Ok(Token {
			kind: TokenKind::Literal(LiteralKind::Float),
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

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Token, LexerError>;

	fn next(&mut self) -> Option<Self::Item> {
		self.skip_whitespaces();
		self.skip_comments();

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
			_ => return Some(self.lex_indent()),
		};

		Some(Ok(Token { kind }))
	}
}

/// TODO: Create Lexer Errors
#[derive(Debug)]
pub enum LexerError {}

#[cfg(test)]
mod tests {
	use crate::lexer::token::LiteralKind;

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
		assert_eq!(
			l.lex_integer().unwrap().kind,
			TokenKind::Literal(LiteralKind::Int)
		);
	}

	#[test]
	fn lexer_lex_float() {
		let mut l = Lexer::new("69.9");
		assert_eq!(
			l.lex_float().unwrap().kind,
			TokenKind::Literal(LiteralKind::Float)
		);
	}

	#[test]
	fn lexer_lex_float_without_end() {
		let mut l = Lexer::new("0.");
		assert_eq!(
			l.lex_float().unwrap().kind,
			TokenKind::Literal(LiteralKind::Float)
		);
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

	#[test]
	fn lexer_iter() {
		let mut vec: Vec<Token> = vec![];
		let l = Lexer::new("var x = 5;");
		for tok in l {
			vec.push(tok.unwrap());
		}

		assert_eq!(vec[0].kind, TokenKind::KeywordVar);
		assert_eq!(vec[1].kind, TokenKind::Indent("x".to_string()));
		assert_eq!(vec[2].kind, TokenKind::Assign);
		assert_eq!(vec[3].kind, TokenKind::Literal(LiteralKind::Float));
		assert_eq!(vec[4].kind, TokenKind::Semicolon);
	}
}
