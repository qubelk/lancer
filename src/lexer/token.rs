pub struct Token {
	pub kind: TokenKind,
	pub span: TokenSpan,
}

enum TokenKind {
	KeywordVar,
	Indent,
	LiteralInt,
	Assign,
	Semicolon,
	Eof,
}

struct TokenSpan {
	start: usize,
	end: usize,
}
