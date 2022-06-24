//--> Imports <--

use crate::{
	Error,
	ErrorList,
	interpreter::{
		InterpretError,
		lexer::{
			Token,
			TokenStream,
			TokenWrapper,
			Lit,
			Op,
			Word,
		}
	},
};

use super::ParseError;

use std::{
	path::{
		Path,
		PathBuf,
	},
};

use logos::Span;

//--> Type Aliases <--

pub(crate) type Result = std::result::Result<(ConcreteSyntaxTree, ErrorList), ErrorList>;

//--> Structs <--

pub(crate) struct ConcreteSyntaxTree {
	pub file: PathBuf,
	pub children: Vec<ConcreteSyntaxNode>
}

pub(crate) struct ConcreteSyntaxNode {
	pub token: Token,
	pub line: usize,
	pub span: Span,
	pub left_children: Option<Vec<ConcreteSyntaxNode>>,
	pub right_children: Option<Vec<ConcreteSyntaxNode>>,
}

//--> Enums <--

enum Context {
	FunctionSignature,
	FunctionCode,
	StructSignature,
	StructContents,
	EnumSignature,
	EnumContents,
}

//--> Functions <--

impl ConcreteSyntaxTree {
	pub(crate) fn generate(p: &Path, mut tokens: TokenStream) -> Result {
		let mut tree = ConcreteSyntaxTree { file: PathBuf::from(p), children: Vec::new() };
		let mut errs: ErrorList = Vec::new();

		let mut ctx_stack: Vec<Context> = Vec::new();

		for (lno, token_line) in tokens.split_mut(|t| if let Token::Operator(Op::Newline) = t.inner { true } else { false }).enumerate() {
			// The tokens in each line are to be used as a stack, so we'll reverse them and turn them into Vecs so we can `.pop()`
			token_line.reverse();
			let mut token_stack = Vec::from(token_line);
			
			if ctx_stack.is_empty() {
				// We are in the top-level context.
			} else {
				// We have a context.
			}
		}

		if errs.is_empty() || errs.iter().all(|e| e.is_warning()) {
			Ok((tree, errs))
		} else {
			Err(errs)
		}
	}
}