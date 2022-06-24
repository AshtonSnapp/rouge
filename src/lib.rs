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
	pub(crate) fn new(is_warning: bool, file: Option<&Path>, line: Option<usize>, span: Option<Span>, kind: ErrorKind) -> Error {
		Error {
			is_warning,
			file: match file {
				Some(p) => Some(PathBuf::from(p)),
				None => None
			},
			line,
			span,
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

	/// Returns the kind of error that occurred.
	#[no_mangle]
	pub extern "C" fn kind(&self) -> ErrorKind {
		self.kind.clone()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
		let mut message = String::new();
		
		if self.is_warning {
			message.push_str("WARN: ");
		} else {
			message.push_str("ERR: ");
		}

		if let Some(filename) = &self.file {
			message.push_str(format!("{}: ", filename.display()));
		} else {
			message.push_str("stdin: ");
		}

		if let Some(lno) = &self.line {
			message.push_str(format!("{}: ", lno));
		}

		if let Some(span) = &self.span {
			message.push_str(format!("{}..{}: ", span.start, span.end));
		}

		match self.kind {
			ErrorKind::Interpret(interp_err) => match interp_err {
				InterpretError::Lex(lex_err) => match lex_err {
					LexError::InvalidToken => message.push_str("Tried to turn something into a token, but failed to."),
					LexError::NumberParseFail => message.push_str("Tried to turn something into a number literal, but failed to."),
					LexError::UnknownEscapeSequence => message.push_str("Tried to turn something into a text literal, but encountered an unknown escape sequence."),
					LexError::InvalidEscapeSequence => message.push_str("Tried to turn something into a text literal, but encountered an invalid escape sequence."),
				},
				InterpretError::Parse(parse_err) => match parse_err {
					ParseError::UnexpectedToken => message.push_str("Ran into an unexpected token while parsing."),
				},
			},
			ErrorKind::Compile => unreachable!(),
			ErrorKind::Load => unreachable!(),
			ErrorKind::IO(io_err) => match io_err {
				IOError::NotFound => message.push_str("Tried to do something, but couldn't find the file."),
				IOError::PermissionDenied => message.push_str("Tried to do something, but didn't have permission."),
				IOError::BrokenPipe => message.push_str("Tried to do something, but a pipe broke."),
				IOError::InvalidInput => message.push_str("Tried to do something, but did it wrong."),
				IOError::InvalidData => message.push_str("Tried to do something, but got nonsense."),
				IOError::TimedOut => message.push_str("Tried to do something, but ran out of time."),
				IOError::WriteZero => message.push_str("Tried to do something, but nothing was written."),
				IOError::Interrupted => message.push_str("Tried to do something, but was interrupted."),
				IOError::Unsupported => message.push_str("Tried to do something, but it's impossible."),
				IOError::UnexpectedEof => message.push_str("Tried to do something, but unexpectedly reached the end of the file."),
				IOError::OutOfMemory => message.push_str("Tried to do something, buut ran out of memory."),
				_ => message.push_str(format!("Tried to do something, but encountered this unexpected error: {}", io_err))
			},
			ErrorKind::Runtime => unreachable!(),
		}

		write!(fmtr, "{}", message)
	}
}