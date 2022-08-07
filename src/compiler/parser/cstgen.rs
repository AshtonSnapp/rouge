//--> Imports <--

use crate::{
	Error,
	ErrorKind,
	ErrorList,
	compiler::{
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

//--> Type Aliases <--

pub(crate) type Result = std::result::Result<(ConcreteSyntaxTree, ErrorList), ErrorList>;

//--> Structs <--

pub(crate) struct ConcreteSyntaxTree {
	pub file: PathBuf,
	pub children: Vec<ConcreteSyntaxNode>
}

pub(crate) struct ConcreteSyntaxNode {
	pub token: TokenWrapper,
	pub lhs: Option<Vec<ConcreteSyntaxNode>>,
	pub rhs: Option<Vec<ConcreteSyntaxNode>>,
}

//--> Enums <--

enum Context {
	FunctionSig,
	FunctionCode,
	ClosureSig,
	ClosureCode,
	StructSig,
	StructContents,
	EnumSig,
	EnumContents,
	TraitSig,
	TraitContents,
	ImplSig,
	ImplContents,
	IfExprSig,
	ElifExprSig,
	MatchArmSig,
	WhileSig,
	UntilSig,
	ForSig,
	CtrlFlowBlock,
	VarDeclaration,
	MutVarDeclaration,
	ConstDeclaration,
	FunctionCall,
	TupleInit,
	CollectionInit,
	StructInit,
	UseTree,
	DecoratorLine,
	CommentLine,
	Public,
	Protected,
	Extern,
}

//--> Functions <--

impl ConcreteSyntaxTree {
	pub(crate) fn generate(p: &Path, mut tokens: TokenStream) -> Result {
		let mut tree = ConcreteSyntaxTree { file: PathBuf::from(p), children: Vec::new() };
		let mut errs: ErrorList = Vec::new();

		let mut ctx_stack: Vec<(Context, ConcreteSyntaxNode)> = Vec::new();

		// doesn't make sense to waste time doing stuff if we have no tokens
		// at the same time, this isn't a fatal error
		if tokens.is_empty() {
			errs.push(Error::new(true, Some(p), None, None, None, ErrorKind::Interpret(InterpretError::Parse(ParseError::NoTokens))));
			return Ok((tree, errs))
		}

		// tokens is being used as a stack
		tokens.reverse();

		let mut lno: usize = 0;

		loop {
			// We know we'll have something because we know it ain't empty.
			let tok = tokens.pop().unwrap();

			if ctx_stack.is_empty() {
				// We are in the file/module top-level context.

				match tok.clone().inner {
					Token::Literal(_) => errs.push(Error::new(
						false,
						Some(p),
						Some(lno),
						Some(tok.span),
						Some(&tok.slice),
						ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
					)),
					Token::Operator(o) => match o {
						Op::Decorator => ctx_stack.push((
							Context::DecoratorLine,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Op::Comment => ctx_stack.push((
							Context::CommentLine,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Op::Newline => lno += 1,
						_ => errs.push(Error::new(
							false,
							Some(p),
							Some(lno),
							Some(tok.span),
							Some(&tok.slice),
							ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
						))
					},
					Token::Keyword(w) => match w {
						Word::Constant => ctx_stack.push((
							Context::ConstDeclaration,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Function => ctx_stack.push((
							Context::FunctionSig,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Structure => ctx_stack.push((
							Context::StructSig,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Enumeration => ctx_stack.push((
							Context::EnumSig,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Trait => ctx_stack.push((
							Context::TraitSig,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Implementation => ctx_stack.push((
							Context::ImplSig,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Public => ctx_stack.push((
							Context::Public,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Protected => ctx_stack.push((
							Context::Protected,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::External => ctx_stack.push((
							Context::Extern,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						Word::Use => ctx_stack.push((
							Context::UseTree,
							ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: Some(Vec::new())
							}
						)),
						_ => errs.push(Error::new(
							false,
							Some(p),
							Some(lno),
							Some(tok.span),
							Some(&tok.slice),
							ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
						))
					},
					Token::Error => unreachable!()
				}
			} else {
				let mut ctx = ctx_stack.pop().unwrap();

				match ctx.0 {
					Context::FunctionSig => {},
					Context::FunctionCode => {},
					Context::ClosureSig => {},
					Context::ClosureCode => {},
					Context::StructSig => {},
					Context::StructContents => {},
					Context::EnumSig => {},
					Context::EnumContents => {},
					Context::TraitSig => {},
					Context::TraitContents => {},
					Context::ImplSig => {},
					Context::ImplContents => {},
					Context::IfExprSig => {},
					Context::ElifExprSig => {},
					Context::MatchArmSig => {},
					Context::WhileSig => {},
					Context::UntilSig => {},
					Context::ForSig => {},
					Context::CtrlFlowBlock => {},
					Context::VarDeclaration => {},
					Context::MutVarDeclaration => {},
					Context::ConstDeclaration => {},
					Context::FunctionCall => {},
					Context::TupleInit => {},
					Context::CollectionInit => {},
					Context::StructInit => {},
					Context::UseTree => {
						match tok.clone().inner {
							Token::Keyword(w) => match w {
								Word::Selff | Word::Super | Word::Identifier(_) => {},
								_ => errs.push(Error::new(
									false,
									Some(p),
									Some(lno),
									Some(tok.span),
									Some(&tok.slice),
									ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
								))
							},
							Token::Operator(o) => match o {
								Op::OpenSquare => {
									ctx_stack.push(ctx);
									ctx_stack.push((
										Context::CollectionInit,
										ConcreteSyntaxNode {
											token: tok,
											lhs: None,
											rhs: Some(Vec::new())
										}
									))
								},
								Op::Colon => if let Some(tok1) = tokens.pop() {} else {},
								Op::Semicolon => {},
								Op::Newline => {},
								_ => errs.push(Error::new(
									false,
									Some(p),
									Some(lno),
									Some(tok.span),
									Some(&tok.slice),
									ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
								))
							},
							Token::Error => unreachable!(),
							_ => errs.push(Error::new(
								false,
								Some(p),
								Some(lno),
								Some(tok.span),
								Some(&tok.slice),
								ErrorKind::Interpret(InterpretError::Parse(ParseError::UnexpectedToken))
							))
						}
					},
					Context::DecoratorLine => {
						match tok.clone().inner {
							Token::Operator(o) => match o {
								Op::Comment => {
									if ctx_stack.is_empty() {
										tree.children.push(ctx.1);
									} else {
										let mut ctx1 = ctx_stack.pop().unwrap();

										match ctx1.0 {
											_ => todo!()
										}
									}

									ctx_stack.push((
										Context::CommentLine,
										ConcreteSyntaxNode {
											token: tok,
											lhs: None,
											rhs: Some(Vec::new())
										}
									));
								},
								Op::Newline => {
									if ctx_stack.is_empty() {
										tree.children.push(ctx.1);
									} else {
										let mut ctx1 = ctx_stack.pop().unwrap();

										match ctx1.0 {
											_ => todo!()
										}
									}

									lno += 1;
								},
								_ => ctx.1.rhs.unwrap().push(ConcreteSyntaxNode {
									token: tok,
									lhs: None,
									rhs: None,
								}),
							},
							Token::Error => unreachable!(),
							_ => ctx.1.rhs.unwrap().push(ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: None,
							}),
							
						}
					},
					Context::CommentLine => {
						match tok.clone().inner {
							Token::Operator(Op::Newline) => {
								if ctx_stack.is_empty() {
										tree.children.push(ctx.1);
									} else {
										let mut ctx1 = ctx_stack.pop().unwrap();

										match ctx1.0 {
											_ => todo!()
										}
									}

									lno += 1;
							},
							_ => ctx.1.rhs.unwrap().push(ConcreteSyntaxNode {
								token: tok,
								lhs: None,
								rhs: None,
							})
						}
					},
					Context::Public => {},
					Context::Protected => {},
					Context::Extern => {},
				}
			}

			if tokens.is_empty() { break; }
		}

		if errs.iter().all(|e| e.is_warning()) {
			Ok((tree, errs))
		} else {
			Err(errs)
		}
	}
}