pub struct Token {
	pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	KeywordVar,
	Indent(String),
	LiteralInt(i64),
	LiteralFloat(f64),
	Assign,
	Semicolon,
	Eof,
}
