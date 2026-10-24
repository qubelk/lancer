use crate::lexer::token::Token;

pub struct ParserCursor {
	tokens: Vec<Token>,
	pos: usize,
}

impl ParserCursor {
	fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, pos: 0 }
	}

	pub fn pos(&self) -> usize {
		self.pos
	}

	pub fn peek(&mut self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	pub fn next(&mut self) -> Option<Token> {
		let t = self.peek().cloned();
		self.pos += 1;
		t
	}

	pub fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
		if self.peek() == Some(expected) {
			self.pos += 1;
			Ok(())
		} else {
			Err(ParseError::Unexpected(
				self.peek().cloned(),
				*expected,
				self.pos,
			))
		}
	}
}

pub enum ParseError {
	// Obtained, Expected, Position
	Unexpected(Option<Token>, Token, usize),
}
