//--> Imports <--

mod lexer;
mod code_parser;
mod obj_parser;

use std::default::Default;
use lexer::{Token, Lit, Op, Wrd, TokenStream};
use std::fmt;
use std::io::ErrorKind as LoadErr;
use std::io::stdin;
use std::path::Path;

//--> Type Aliases <--

pub type ErrorList = Vec<Error>;

//--> Structs <--

#[derive(Clone)]
pub struct Runtime {
	hooks: Vec<Hook>,
	stack: Vec<Data>
}

pub struct Error {
	message: String,
	is_warning: bool,
	info: ErrorInfo
}

pub struct CompileOptions {
	optimize: u8,
	compress: i8,
	warn_is_err: bool
}

#[derive(PartialEq, Eq, Clone)]
pub struct StructField {}

#[derive(PartialEq, Eq, Clone)]
pub struct EnumVariant {}

#[derive(Clone)]
pub struct FuncPtr {}

#[derive(Clone)]
pub struct HeapPtr {}

//--> Enums <--

#[derive(Clone, Copy)]
pub enum ErrorInfo {
	Load(LoadErr),
	Parse(ParseErr),
	Run(RunErr)
}

#[derive(Clone, Copy)]
pub enum ParseErr {
	InvalidToken,
}

#[derive(Clone, Copy)]
pub enum RunErr {}

#[derive(PartialEq, Eq, Clone)]
pub enum DataType {
	U1,
	I1,
	U2,
	I2,
	U4,
	I4,
	F4,
	U8,
	I8,
	F8,
	B,
	C,
	T(Vec<DataType>),
	F,
	P(String),
}

#[derive(Clone)]
enum Data {
	U1(u8),
	I1(i8),
	U2(u16),
	I2(i16),
	U4(u32),
	I4(i32),
	F4(f32),
	U8(u64),
	I8(i64),
	F8(f64),
	B(bool),
	C(char),
	T(Vec<Data>),
	F(FuncPtr),
	P(HeapPtr),
}

#[derive(PartialEq, Eq, Clone)]
enum Hook {
	Function{
		name: String,
		args: Vec<DataType>,
		rets: DataType,
		func: fn()
	},
	Struct{
		name: String,
		fields: Vec<StructField>,
		converter: fn()
	},
	Enum{
		name: String,
		variants: Vec<EnumVariant>,
		converter: fn()
	}
}

//--> Functions <--

impl Runtime {
	pub fn new() -> Runtime { Runtime { hooks: Vec::new(), stack: Vec::new() } }

	pub fn hook_function(&mut self, name: String, args: Vec<DataType>, rets: DataType, func: fn()) -> Runtime {
		let hook = Hook::Function{ name, args, rets, func };

		if !self.hooks.contains(&hook) {
			self.hooks.push(hook);
		}

		self.clone()
	}

	pub fn hook_struct(&mut self, name: String, fields: Vec<StructField>, converter: fn()) -> Runtime {
		let hook = Hook::Struct{ name, fields, converter };

		if !self.hooks.contains(&hook) {
			self.hooks.push(hook);
		}

		self.clone()
	}

	pub fn hook_enum(&mut self, name: String, variants: Vec<EnumVariant>, converter: fn()) -> Runtime {
		let hook = Hook::Enum{ name, variants, converter };

		if !self.hooks.contains(&hook) {
			self.hooks.push(hook);
		}

		self.clone()
	}

	pub fn load(&mut self, path: &Path, includes: Vec<&Path>) -> Result<(Option<FuncPtr>, ErrorList), ErrorList> {
		let mut func = None;
		let mut errs = Vec::new();
		let mut wrns = Vec::new();

		let mut toks: Vec<TokenStream> = Vec::new();

		match Token::lex_file(path) {
			Ok(t) => toks.push(t),
			Err(mut e) => errs.append(&mut e)
		}

		if errs.is_empty() {
			Ok((func, wrns))
		} else {
			errs.append(&mut wrns);
			Err(errs)
		}
	}

	pub fn compile(&self, paths: Vec<&Path>, includes: Vec<&Path>, outfile: &Path, opts: CompileOptions) -> Result<ErrorList, ErrorList> {
		let mut errs = Vec::new();
		let mut wrns = Vec::new();

		let actual_outfile = outfile.with_extension("robj").as_path();

		let mut toks: Vec<TokenStream> = Vec::new();

		for path in paths {
			match Token::lex_file(path) {
				Ok(t) => toks.push(t),
				Err(mut e) => errs.append(&mut e)
			}
		}

		if errs.is_empty() {
			Ok(wrns)
		} else {
			errs.append(&mut wrns);
			Err(errs)
		}
	}

	pub fn compile_and_load(&mut self, paths: Vec<&Path>, includes: Vec<&Path>, outfile: &Path, opts: CompileOptions) -> Result<(Option<FuncPtr>, ErrorList), ErrorList> {
		let mut func = None;
		let mut errs = Vec::new();
		let mut wrns = Vec::new();

		let actual_outfile = outfile.with_extension("robj").as_path();

		let mut toks: Vec<TokenStream> = Vec::new();

		for path in paths {
			match Token::lex_file(path) {
				Ok(t) => toks.push(t),
				Err(mut e) => errs.append(&mut e)
			}
		}

		if errs.is_empty() {
			Ok((func, wrns))
		} else {
			errs.append(&mut wrns);
			Err(errs)
		}
	}

	pub fn repl(&mut self) {
		let mut lno: usize = 0;
		let mut blocks_deep: usize = 0;
		println!("Rouge REPL: Type end or press Ctrl+C to exit.");
		loop {
			let mut input = String::new();
			print!("stdin[{}]=> ", lno);
			match stdin().read_line(&mut input) {
				Ok(_) => match Token::lex_line(&input, lno) {
					Ok(t) => {
						if blocks_deep == 0 {
							if let Token::Word(Wrd::EndBlock) = t[0] {
								break;
							}
						}
						// TODO: implement
					},
					Err(errs) => for err in errs {
						eprintln!("{}", err.message());
					}
				},
				Err(e) => match e.kind() {
					LoadErr::InvalidData => eprintln!("<ERR! stdin:{}> Input data contains invalid UTF-8.", lno),
					_ => eprintln!("<ERR! stdin:{}> Unexpected I/O error '{:?}' encountered trying to read input.", lno, e.kind())
				}
			}
			lno += 1;
		}
	}
}

impl Error {
	pub fn new(message: String, is_warning: bool, info: ErrorInfo) -> Error { Error { message, is_warning, info } }

	pub fn message(&self) -> &str { &self.message }

	pub fn warning(&self) -> bool { self.is_warning }

	pub fn info(&self) -> ErrorInfo { self.info }
}

impl fmt::Display for Error {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
		write!(fmtr, "{}", self.message)
	}
}

impl CompileOptions {
	pub fn new(optimize: u8, compress: i8, warn_is_err: bool) -> CompileOptions { CompileOptions { optimize, compress, warn_is_err } }

	pub fn optimize_level(&self) -> u8 { self.optimize }

	pub fn compress_level(&self) -> i32 { self.compress as i32 }

	pub fn warn_is_err(&self) -> bool { self.warn_is_err }
}

impl Default for CompileOptions {
	fn default() -> CompileOptions { CompileOptions { optimize: 0, compress: 20, warn_is_err: false } }
}