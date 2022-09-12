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

//--> Structs <--

pub(crate) struct ConcreteSyntaxTree {
	pub file: PathBuf,
	pub children: Vec<ConcreteSyntaxNode>
}

pub(crate) struct ConcreteSyntaxNode {
	pub token: Token,
	pub lhs: Option<OneOrMany<ConcreteSyntaxNode>>,
	pub rhs: Option<OneOrMany<ConcreteSyntaxNode>>,
}

struct ContextFrame {
	pub ctx: Context,
	pub node: ConcreteSyntaxNode,
}

//--> Enums <--

pub(crate) enum OneOrMany<T> {
	One(Box<T>),
	Many(Vec<T>),
}

/// An enumeration of different contexts. Each context has different rules for what tokens are expected or allowed.
enum Context {
	// start = `use`, end = newline
	UseTree,
	// start = `class`, end = `is` or newline
	ClassSig,
	// start = `enum`, end = `is`
	EnumSig,
	// start = `trait`, end = `is` or newline
	TraitSig,
	// start = `impl`, end = `is` or newline
	ImplSig,
	// start = `func`, end = `do` or newline (dependent on parent context and other syntax)
	FuncSig,
	// start = type keyword or class identifier, end = newline
	VarDeclaration,
	// start = `mut`, end = newline
	MutVarDeclaration,
	// start = `const`, end = newline
	ConstDeclaration,
	// start = `(`, end = `)`
	TupleInitOrArgsOrSupersDeclaration,
	// start = `<`, end = `>`
	GenericsDeclaration,
	// start = `[`, end = `]`
	CollectionInit,
	// start = `if` or `elif`, end = `is` or `then`
	ConditionalSig,
}