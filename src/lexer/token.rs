#[derive(Debug, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: TokenSpan,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
	Int,
	Float,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	KeywordVar,
	Ident,
	Literal(LiteralKind),
	Assign,
	Semicolon,
	Eof,
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
	pub start: usize,
	pub end: usize,
}
