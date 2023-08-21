//! Rouge is a statically-typed programming language designed for two primary uses:
//! applications (graphical and command-line), and embedding into native programs (plugins, config files).
//! To be suitable for both use cases, Rouge aims to have the following feature set:

//--> Imports <--

mod compiler;

use compiler::{
	InterpretError,
};

use std::{
	any::Any,
	collections::{
		BTreeMap,
		HashMap,
		VecDeque,
	},
	io::ErrorKind as IOError,
	path::{
		Path,
		PathBuf
	},
};

use logos::Span;

//--> Type Aliases <--

/// Represents a collection of errors and/or warnings.
pub type ErrorList = Vec<Error>;

//--> Structs <--

/// The Rouge runtime itself.
pub struct Runtime {}

/// Contains information about some kind of error that occurred while trying to run a program.
/// 
/// The expectation is that programs embeding the runtime will use the information contained to generate error messages.
/// However, for prototyping (or laziness), the Display trait is implemented to automatically generate error messages for you.
#[derive(Debug)]
pub struct Error {
	is_warning: bool,
	file: Option<PathBuf>,
	line: Option<usize>,
	span: Option<Span>,
	slice: Option<String>,
	kind: ErrorKind
}

//--> Enums <--

/// Indicates what kind of error has occurred, including any significant information that is specific to a given kind of error.
/// Errors, at a top-level, are grouped into one of the following general categories:
/// 
///  - Interpretation errors, which can occur when trying to figure out what source code means and whether it is valid.
///  - Compilation errors, which can occur when trying to convert interpreted code into bytecode.
///  - Loading errors, which can occur when trying to load a bytecode file.
///  - I/O errors, which can occur when the runtime tries to load a file containing either source code (for interpretation or compilation) or bytecode.
///  - Runtime errors, which occur when something goes wrong while code is trying to run.
#[derive(Clone, Debug)]
pub enum ErrorKind {
	Interpret(InterpretError),
	Compile,
	Load,
	IO(IOError),
	Runtime,
}

pub enum RougeData {
	Nat(u64),
	Int(i64),
	Flo(f64),
	Char(char),
	Bool(bool),
	List(VecDeque<RougeData>),
	Map(BTreeMap<RougeData, RougeData>),
	Str(String),
	Struct(HashMap<Member, RougeData>),
	Union{ tag: u8, data: HashMap<Member, RougeData> },
	ConstRef(),
	HeapRef(),
	ExternData(Box<dyn Any>)
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Member {
	Unnamed(u64),
	Named(String),
}

//--> Functions <--

impl Runtime {
	/// The semantic version of the Rouge runtime, as a tuple.
	pub const ROUGE_VERSION_TUPLE: (u8, u8, u8) = (0, 1, 0);
	/// A human-friendly string representation of the version.
	pub const ROUGE_VERSION_STRING: &'static str = "0.1.0";
	
	pub fn new() -> Runtime {
		Runtime {}
	}
}

impl Error {
	/// Creates a new error object.
	pub(crate) fn new(is_warning: bool, file: Option<&Path>, line: Option<usize>, span: Option<Span>, slice: Option<&str>, kind: ErrorKind) -> Error {
		Error {
			is_warning,
			file: file.map(|path| path.to_path_buf()),
			line,
			span,
			slice: slice.map(|source| source.to_string()),
			kind
		}
	}

	/// Indicates whether this is a full-on error, or a simple warning.
	pub fn is_warning(&self) -> bool {
		self.is_warning
	}

	/// Indicates where the error came from.
	/// A return value of None indicates the error came from the REPL.
	pub fn file(&self) -> Option<PathBuf> {
		self.file.clone()
	}

	/// Indicates what line the error came from.
	/// A return value of None indicates that this error applies to the entire file.
	pub fn line(&self) -> Option<usize> {
		self.line
	}

	/// Indicates what characters generated the error.
	/// A return value of None indicates the error applies to the entire line (or file if a line number is unspecified).
	pub fn span(&self) -> Option<Span> {
		self.span.clone()
	}

	/// Contains the text that generated the error.
	/// A return value of None indicates the error doesn't apply to a specific string of text.
	pub fn slice(&self) -> Option<String> {
		self.slice.clone()
	}

	/// Returns the kind of error that occurred.
	pub fn kind(&self) -> ErrorKind {
		self.kind.clone()
	}
}