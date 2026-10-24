pub enum LiteralKind {
	Int(i64),
	Float(f64),
}

pub enum Expr {
	Literal(LiteralKind),
	Ident(String),
	Binary {
		op: BinaryOp,
		left: Box<Expr>,
		right: Box<Expr>,
	},
}

pub enum BinaryOp {
	Assign,
}

pub enum Stmt {
	Var { ident: String, value: Expr },
	Expr(Expr),
}
