pub struct Token {
	pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
	Int,
	Float,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	KeywordVar,
	Indent(String),
	Literal(LiteralKind),
	Assign,
	Semicolon,
}
