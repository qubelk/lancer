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

		let lexer = lexer::Lexer::new(&data);
		for token in lexer {
			let _ = dbg!(token);
		}
	}
}
