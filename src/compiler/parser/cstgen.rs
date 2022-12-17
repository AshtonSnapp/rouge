//--> Imports <--

use crate::{
	Error,
	ErrorKind,
	ErrorList,
	compiler::{
		InterpretError,
		lexer::{
			Token,
			TokenInner,
			TokenStream,
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

//--> Type Aliases <--

pub(crate) type Result = std::result::Result<(ConcreteSyntaxTree, ErrorList), ErrorList>;

type ContextStack = Vec<ConcreteSyntaxNode>;

//--> Structs <--

#[derive(Clone, Debug)]
pub(crate) struct ConcreteSyntaxTree {
	pub file: PathBuf,
	pub root: ConcreteSyntaxNode
}

#[derive(Clone, Debug)]
pub(crate) struct ConcreteSyntaxNode {
	pub token: Token,
	pub lhs: Option<Box<ConcreteSyntaxNode>>,
	pub rhs: Option<Box<ConcreteSyntaxNode>>,
}

//--> Functions <--

impl ConcreteSyntaxTree {
	pub(crate) fn new(path: &Path, mut tokens: TokenStream) -> Result {
		if tokens.is_empty() {
			return Err(vec![
				// TODO: I should probably simplify how ErrorKind works. Nesting enums seemed like a good idea at first, but not any more...
				Error::new(false, Some(path), None, None, None, ErrorKind::Interpret(InterpretError::Parse(ParseError::NoTokens)))
			])
		}
		
		let file = path.to_path_buf();
		let mut errors = ErrorList::new();

		// Going to use the tokens as a stack.
		tokens.reverse();

		let mut context = ContextStack::new();

		loop {
			let node = ConcreteSyntaxNode::new(tokens.pop().unwrap());

			// TODO: How the fuck do I write a parser????

			context.push(node);

			if tokens.is_empty() { break; }
		}

		if errors.is_empty() || errors.iter().all(|e| e.is_warning()) {
			Ok((ConcreteSyntaxTree {
				file,
				root: context.pop().unwrap()
			}, errors))
		} else {
			Err(errors)
		}
	}
}

impl ConcreteSyntaxNode {
	pub(crate) fn new(token: Token) -> ConcreteSyntaxNode {
		ConcreteSyntaxNode {
			token,
			lhs: None,
			rhs: None,
		}
	}

	pub(crate) fn lefthand(self, lhs: ConcreteSyntaxNode) -> ConcreteSyntaxNode {
		ConcreteSyntaxNode {
			lhs: Some(Box::new(lhs)),
			..self
		}
	}

	pub(crate) fn righthand(self, rhs: ConcreteSyntaxNode) -> ConcreteSyntaxNode {
		ConcreteSyntaxNode {
			rhs: Some(Box::new(rhs)),
			..self
		}
	}

	pub(crate) fn with_lefthand(&mut self, lhs: ConcreteSyntaxNode) {
		self.lhs = Some(Box::new(lhs));
	}

	pub(crate) fn with_righthand(&mut self, rhs: ConcreteSyntaxNode) {
		self.rhs = Some(Box::new(rhs));
	}
}