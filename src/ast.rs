#[derive(Debug, PartialEq)]
pub enum AstLiteralKind {
	Int(i64),
	Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
	Literal(AstLiteralKind),
	Ident(String),
	Binary {
		op: BinaryOp,
		left: Box<Expr>,
		right: Box<Expr>,
	},
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
	Assign,
}

#[derive(Debug)]
pub enum Stmt {
	Var { ident: String, value: Option<Expr> },
	Expr(Expr),
}
