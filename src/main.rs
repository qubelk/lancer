mod ast;
mod lexer;
mod parser;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	for arg in &args[1..] {
		let path = Path::new(arg);
		let mut file = match File::open(path) {
			Ok(f) => f,
			Err(e) => panic!("Couldn't open {}: {}", path.display(), e),
		};

		let mut data = String::new();
		if let Err(e) = file.read_to_string(&mut data) {
			panic!("Failed to read {}: {}", path.display(), e);
		}

		let mut lexer = lexer::Lexer::new(&data);
		let tokens = lexer.tokenize().unwrap();
		let _ = dbg!(&tokens);

		let mut parser = parser::Parser::new(tokens);
		let _ = dbg!(parser.parse());
	}
}
