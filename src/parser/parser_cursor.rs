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

	pub fn peek(&self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	pub fn peek_kind(&self) -> Option<&TokenKind> {
		Some(&self.peek()?.kind)
	}

	pub fn next(&mut self) -> Option<Token> {
		let t = self.peek().cloned();
		self.pos += 1;
		t
	}

	pub fn expect(&mut self, kind: &TokenKind, expected: Expected) -> Result<(), ParseError> {
		if self.peek_kind() == Some(kind) {
			self.pos += 1;
			Ok(())
		} else {
			Err(ParseError::Unexpected(self.peek_kind().cloned(), expected))
		}
	}
}

pub enum Expected {
	Kind(TokenKind),
	Literal,
}

pub enum ParseError {
	Unexpected(Option<TokenKind>, Expected),
	ExpectedIdent,
}
