//! Rouge is a statically-typed programming language designed for two primary uses:
//! applications (graphical and command-line), and embedding into native programs (plugins, config files).
//! To be suitable for both use cases, Rouge aims to have the following feature set:
//! 
//!  - A memory management model that aims to be intuitive and performant but with at least some guarantees towards memory and thread safety.
//!  - A simple, easy-to-learn syntax inspired primarily by Ruby and Lua.
//!  - Interpreted for development and use in config files, bytecode-compiled for distribution.
//! 
//! The Rouge runtime is developed in Rust and is designed to be embeddable in other programs.
//! This is done by allowing embedding programs to add _hooks_, functions that can be called by programs running on top of the runtime,
//! and by adding a trait that allows custom types to be put into and taken out of the runtime.
//! 
//! It should also be embeddable in programs written in languages other than Rust, which is accomplished by making most functions use the C ABI (via `extern "C"`).
//! I say 'most' because, due to how Rust considers all non-Rust code unsafe, the code for making a safe hook does not use the C ABI.

//--> Imports <--

mod interpreter;

use interpreter::{
	InterpretError,
	LexError,
	ParseError,
};

use std::{
	fmt,
	io::ErrorKind as IOError,
	path::{
		Path,
		PathBuf
	},
};

use logos::Span;

//--> Type Aliases <--

pub type ErrorList = Vec<Error>;

//--> Structs <--

/// The Rouge runtime itself.
#[repr(C)]
pub struct Runtime;

/// Contains information about some kind of error that occurred while trying to run a program.
/// 
/// The expectation is that programs embeding the runtime will use the information contained to generate error messages.
/// However, for prototyping (or laziness), the Display trait is implemented to automatically generate error messages for you.
#[repr(C)]
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
/// 
/// For embedders developing in a non-Rust programming language, treat this like a tagged union if at all possible.
/// If that is not possible, you are up a creek without a paddle as far as I can see.
#[repr(u8)]
#[derive(Clone)]
pub enum ErrorKind {
	Interpret(InterpretError),
	Compile,
	Load,
	IO(IOError),
	Runtime,
}

//--> Traits <--

//--> Functions <--

impl Runtime {
	#[no_mangle]
	pub extern "C" fn new() -> Runtime {
		Runtime {}
	}
}

impl Error {
	/// Creates a new error object.
	pub(crate) fn new(is_warning: bool, file: Option<&Path>, line: Option<usize>, span: Option<Span>, slice: Option<&str>, kind: ErrorKind) -> Error {
		Error {
			is_warning,
			file: match file {
				Some(p) => Some(PathBuf::from(p)),
				None => None
			},
			line,
			span,
			slice: match slice {
				Some(s) => Some(String::from(s)),
				None => None
			},
			kind
		}
	}

	/// Indicates whether this is a full-on error, or a simple warning.
	#[no_mangle]
	pub extern "C" fn is_warning(&self) -> bool {
		self.is_warning
	}

	/// Indicates where the error came from.
	/// A return value of None indicates the error came from the REPL.
	#[no_mangle]
	pub extern "C" fn file(&self) -> Option<PathBuf> {
		self.file.clone()
	}

	/// Indicates what line the error came from.
	/// A return value of None indicates that this error applies to the entire file.
	#[no_mangle]
	pub extern "C" fn line(&self) -> Option<usize> {
		self.line
	}

	/// Indicates what characters generated the error.
	/// A return value of None indicates the error applies to the entire line (or file if a line number is unspecified).
	#[no_mangle]
	pub extern "C" fn span(&self) -> Option<Span> {
		self.span.clone()
	}

	/// Contains the text that generated the error.
	/// A return value of None indicates the error doesn't apply to a specific string of text.
	#[no_mangle]
	pub extern "C" fn slice(&self) -> Option<String> {
		self.slice.clone()
	}

	/// Returns the kind of error that occurred.
	#[no_mangle]
	pub extern "C" fn kind(&self) -> ErrorKind {
		self.kind.clone()
	}
}