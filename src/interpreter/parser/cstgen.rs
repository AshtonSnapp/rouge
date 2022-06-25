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
}

//--> Functions <--

impl ConcreteSyntaxTree {
	pub(crate) fn generate(p: &Path, mut tokens: TokenStream) -> Result {
		let mut tree = ConcreteSyntaxTree { file: PathBuf::from(p), children: Vec::new() };
		let mut errs: ErrorList = Vec::new();

		let mut ctx_stack: Vec<Context> = Vec::new();

		for (lno, token_line) in tokens.split_mut(|t| if let Token::Operator(Op::Newline) = t.inner { true } else { false }).enumerate() {
			// skip the line if it doesn't contain anything
			if token_line.is_empty() { continue; }

			// TODO: Write CST generation code
		}

		if errs.is_empty() || errs.iter().all(|e| e.is_warning()) {
			Ok((tree, errs))
		} else {
			Err(errs)
		}
	}
}