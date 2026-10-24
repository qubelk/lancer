#[derive(Clone, Debug, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: TokenSpan,
	pub orig: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenLiteralKind {
	Int,
	Float,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
	KeywordVar,
	Ident,
	Literal(TokenLiteralKind),
	Assign,
	Semicolon,
	Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenSpan {
	pub start: usize,
	pub end: usize,
}
