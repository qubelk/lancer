use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

pub struct ParserCursor {
	tokens: Vec<Token>,
	pos: usize,
}

impl ParserCursor {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, pos: 0 }
	}

	pub fn pos(&self) -> usize {
		self.pos
	}

	pub fn peek(&mut self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	pub fn peek_kind(&mut self) -> Option<&TokenKind> {
		Some(&self.peek()?.kind)
	}

	pub fn next(&mut self) -> Option<Token> {
		let t = self.peek().cloned();
		self.pos += 1;
		t
	}

	pub fn expect(&mut self, expected: &TokenKind) -> Result<(), ParseError> {
		if self.peek_kind() == Some(expected) {
			self.pos += 1;
			Ok(())
		} else {
			Err(ParseError::Unexpected(
				self.peek_kind().cloned(),
				expected.clone(),
				self.pos,
			))
		}
	}
}

pub enum ParseError {
	// Obtained, Expected, Position
	Unexpected(Option<TokenKind>, TokenKind, usize),
	ExpectedIdent,
}
