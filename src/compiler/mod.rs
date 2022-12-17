//--> Imports <--

mod lexer;
mod parser;

use std::{
	collections::HashMap,
	path::Path,
};

use lexer::{
	Token,
	TokenStream,
};

pub use lexer::LexError;
pub use parser::ParseError;

use crate::Error;

//--> Enums <--

#[derive(Clone, Debug)]
pub enum InterpretError {
	Lex(LexError),
	Parse(ParseError),
}

//--> Functions <--

pub fn compile(paths: Vec<&Path>) {
	// We need at least one path!
	if paths.is_empty() {}

	// At least one path needs to be a file, so we don't go rooting around in directories until we absolutely need to.
	if !paths.iter().any(|p| p.is_file()) {}

	let mut errs: Vec<Error> = Vec::new();

	let mut tok_files: HashMap<&Path, TokenStream> = HashMap::new();

	for file_path in paths.iter().filter(|p| p.is_file()) {
		match Token::lex_file(file_path) {
			Ok(t) => if !t.is_empty() {
				tok_files.insert(file_path, t);
			},
			Err(e) => {
				for err in e {
					errs.push(err);
				}
			}
		}
	}

	// If we have any errors, return early.
	if !errs.is_empty() {}
}