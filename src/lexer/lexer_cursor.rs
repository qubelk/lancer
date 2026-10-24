#[derive(Clone)]
pub struct LexerCursor<'a> {
	input: &'a str,
	pos: usize,
}

impl<'a> LexerCursor<'a> {
	pub fn new(input: &'a str) -> Self {
		Self { input, pos: 0 }
	}

	pub fn next(&mut self) -> Option<char> {
		let ch = self.peek()?;
		self.pos += ch.len_utf8();
		Some(ch)
	}

	pub fn peek(&self) -> Option<char> {
		self.input[self.pos..].chars().next()
	}

	pub fn peekn(&self, n: usize) -> Option<char> {
		self.input[self.pos..].chars().nth(n)
	}

	pub fn pos(&self) -> usize {
		self.pos
	}

	pub fn slice(&self, start: usize, end: usize) -> String {
		self.input[start..end].to_string()
	}
}
