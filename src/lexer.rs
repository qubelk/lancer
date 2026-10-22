use crate::lexer::cursor::Cursor;

mod cursor;
mod token;

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
