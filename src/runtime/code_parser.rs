//--> Imports <--

use super::lexer::{Token, TokenStream, Lit, Op, Wrd};
use super::{Error, ErrorInfo, ParseErr, ErrorList};
use std::path::Path;

//--> Type Aliases <--

pub type ASTree = Vec<ASTNode>;

pub type ASTResult = Result<(ASTree, ErrorList), ErrorList>;

//--> Structs <--

pub struct ASTNode {
	contents: ASTNodeType,
	children: Vec<ASTNode>,
	src_toks: TokenStream
}

//--> Enums <--

pub enum ASTNodeType {}

//--> Functions <--

pub fn parse(toks: TokenStream, p: &Path, werr: bool) -> ASTResult {
	let mut tree = Vec::new();
	let mut errs = Vec::new();
	let mut wrns = Vec::new();

	let mut tokstack = toks.iter().rev();

	'outer: loop {
		let mut tline: TokenStream = Vec::new();

		loop {
			let tok = match tokstack.next() { Some(t) => t, None => break 'outer };
			if let Token::Newline = tok {
				break;
			}
			tline.push(tok.clone());
		}

		
	}

	if errs.is_empty() {
		Ok((tree, wrns))
	} else {
		errs.append(&mut wrns);
		Err(errs)
	}
}