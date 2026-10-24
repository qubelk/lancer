#[derive(Clone, Debug, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: TokenSpan,
	pub orig: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind {
	Int,
	Float,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
	KeywordVar,
	Ident,
	Literal(LiteralKind),
	Assign,
	Semicolon,
	Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenSpan {
	pub start: usize,
	pub end: usize,
}
