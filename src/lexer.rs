mod cursor;
mod token;

use cursor::Cursor;

pub struct Lexer<'a> {
	cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			cursor: Cursor::new(input),
		}
	}
}
