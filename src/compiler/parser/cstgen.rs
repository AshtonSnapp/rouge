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

type ContextStack = Vec<ContextFrame>;

//--> Structs <--

pub(crate) struct ConcreteSyntaxTree {
	pub file: PathBuf,
	pub children: Vec<ConcreteSyntaxNode>
}

#[derive(Clone)]
pub(crate) struct ConcreteSyntaxNode {
	pub token: Token,
	pub lhs: Option<Vec<ConcreteSyntaxNode>>,
	pub rhs: Option<Vec<ConcreteSyntaxNode>>,
}

/// ContextFrames are used to temporarily hold CST nodes that initiate a context.
struct ContextFrame {
	pub ctx: Context,
	pub node: ConcreteSyntaxNode,
}

//--> Enums <--

/// An enumeration of different contexts. Each context has different rules for what tokens are expected or allowed.
#[derive(Debug)]
enum Context {
	Public,
	Protected,
	External,
	Comment,
	Decorator,
	UseTree,
}

//--> Functions <--

impl ConcreteSyntaxTree {
	pub(crate) fn new(path: &Path, mut source: TokenStream) -> Result {
		let mut tree = ConcreteSyntaxTree { file: PathBuf::from(path), children: Vec::new() };
		let mut errs: ErrorList = Vec::new();

		if source.is_empty() {
			errs.push(Error::new(
				false,
				Some(path.clone()),
				None,
				None,
				None,
				ErrorKind::Interpret(InterpretError::Parse(ParseError::NoTokens))
			));
			return Err(errs)
		}

		let mut ctx_stack: ContextStack = Vec::new();

		// The stream of source tokens is used as a stack, so reversing is needed.
		source.reverse();

		let mut line_number: usize = 0;

		loop {
			// Unwrapping is safe here.
			// source is guaranteed to have at least one token in it whenever this runs.
			let t0 = source.pop().unwrap();

			let mut end_ctx = false;
			match ctx_stack.last_mut() {
				Some(ctx_frame) => match &ctx_frame.ctx {
					unhandled => {
						todo!("Handling for context {:?} is not implemented yet", unhandled)
					}
				},
				None => {
					// Top-level context
					match &t0.inner {
						TokenInner::Literal(_) => {
							errs.push(Error::new(
								false,
								Some(path.clone()),
								Some(line_number),
								Some(t0.span.clone()),
								Some(&t0.slice.clone()),
								ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
							));
						},
						TokenInner::Operator(op) => match op {
							Op::Decorator => {
								ctx_stack.push(ContextFrame::new(Context::Decorator, ConcreteSyntaxNode::new(t0.clone())));
							},
							Op::Comment => {
								ctx_stack.push(ContextFrame::new(Context::Comment, ConcreteSyntaxNode::new(t0.clone())));
							},
							Op::Newline => {
								line_number += 1;
							},
							_ => {
								errs.push(Error::new(
									false,
									Some(path.clone()),
									Some(line_number),
									Some(t0.span.clone()),
									Some(&t0.slice.clone()),
									ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
								));
							}
						},
						TokenInner::Keyword(word) => match word {
							Word::Public => {
								ctx_stack.push(ContextFrame::new(Context::Public, ConcreteSyntaxNode::new(t0.clone())));
							},
							Word::Protected => {
								ctx_stack.push(ContextFrame::new(Context::Protected, ConcreteSyntaxNode::new(t0.clone())));
							},
							Word::External => {
								ctx_stack.push(ContextFrame::new(Context::External, ConcreteSyntaxNode::new(t0.clone())));
							},
							_ => {
								errs.push(Error::new(
									false,
									Some(path.clone()),
									Some(line_number),
									Some(t0.span.clone()),
									Some(&t0.slice.clone()),
									ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
								));
							}
						},
						TokenInner::Error => unreachable!()
					}
				}
			}

			if source.is_empty() { break }
		}

		if errs.is_empty() || errs.iter().all(|e| e.is_warning()) {
			Ok((tree, errs))
		} else {
			Err(errs)
		}
	}
}

impl ConcreteSyntaxNode {
	/// Creates a new node.
	pub(crate) fn new(token: Token) -> ConcreteSyntaxNode { ConcreteSyntaxNode { token, lhs: None, rhs: None } }

	/// Adds a node to the lefthand side.
	pub(crate) fn add_lefthand(mut self, node: ConcreteSyntaxNode) -> ConcreteSyntaxNode {
		if self.lhs.is_none() {
			self.lhs = {
				let mut vec = Vec::new();
				vec.push(node);
				Some(vec)
			};
		} else {
			self.lhs.as_mut().unwrap().push(node);
		}
		self
	}

	/// Adds a node to the righthand side.
	pub(crate) fn add_righthand(mut self, node: ConcreteSyntaxNode) -> ConcreteSyntaxNode {
		if self.rhs.is_none() {
			self.rhs = {
				let mut vec = Vec::new();
				vec.push(node);
				Some(vec)
			};
		} else {
			self.rhs.as_mut().unwrap().push(node);
		}
		self
	}
}

impl ContextFrame {
	/// Creates a new context frame.
	pub fn new(ctx: Context, node: ConcreteSyntaxNode) -> ContextFrame { ContextFrame { ctx, node } }
}