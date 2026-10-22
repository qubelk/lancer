use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
pub struct Cursor<'a> {
	chars: Peekable<Chars<'a>>,
	pos: usize,
}

impl<'a> Cursor<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			chars: input.chars().peekable(),
			pos: 0,
		}
	}

	pub fn next(&mut self) -> Option<char> {
		let ch = self.chars.next()?;
		self.pos += ch.len_utf8();
		Some(ch)
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.chars.peek()
	}

	pub fn pos(&self) -> usize {
		self.pos
	}
}
