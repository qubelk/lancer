use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
pub struct Cursor<'a> {
	chars: Peekable<Chars<'a>>,
}

impl<'a> Cursor<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			chars: input.chars().peekable(),
		}
	}

	pub fn next(&mut self) -> Option<char> {
		self.chars.next()
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.chars.peek()
	}
}
