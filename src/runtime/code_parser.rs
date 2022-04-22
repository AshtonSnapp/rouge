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

	'lines: for (lno, tline) in toks.split(|tok| if let Token::Newline = tok { true } else { false }).enumerate() {
		let mut tl = tline.iter();

		// TODO: how do I write a parser?
	}

	if errs.is_empty() {
		Ok((tree, wrns))
	} else {
		errs.append(&mut wrns);
		Err(errs)
	}
}