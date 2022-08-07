//--> Imports <--

mod cstgen; // Concrete Syntax Tree Generator (Tokens -> CST)
mod astgen; // Abstract Syntax Tree Generator (CST -> AST)

use crate::{
	Error,
	ErrorList,
};

use super::InterpretError;

use cstgen::ConcreteSyntaxTree;

use std::{
	collections::HashMap,
	path::Path,
};

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

#[derive(Clone)]
#[repr(u8)]
pub enum ParseError {
	/// The parser didn't have anything to parse.
	NoTokens,
	
	/// The parser ran across a token it wasn't expecting.
	UnexpectedToken,
}

//--> Functions <--

pub fn tokens_to_ast() {}