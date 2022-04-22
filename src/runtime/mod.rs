//--> Imports <--

mod lexer;
mod code_parser;
mod obj_parser;

use std::default::Default;
use lexer::{Token, Lit, Op, Wrd, TokenStream};
use std::fmt;
use std::io::ErrorKind as LoadErr;
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

//--> Type Aliases <--

pub type ErrorList = Vec<Error>;

//--> Structs <--

#[derive(Clone)]
pub struct Runtime {
	hooks: Vec<Hook>,
	code: Vec<CodeObject>,
	heap: Vec<HeapObject>,
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

#[derive(Clone)]
struct CodeObject {
	source: PathBuf,
	consts: Vec<Const>,
	code: Vec<Instruction>
}

#[derive(Clone)]
struct HeapObject {
	references: usize,
	data: Vec<Data>
}

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
	/// Unsigned 1-byte (8-bit) integer
	U1,
	/// Signed 1-byte (8-bit) integer
	I1,
	/// Unsigned 2-byte (16-bit) integer
	U2,
	/// Signed 2-byte (16-bit) integer
	I2,
	/// Unsigned 4-byte (32-bit) integer
	U4,
	/// Signed 4-byte (32-bit) integer
	I4,
	/// 4-byte (32-bit) float
	F4,
	/// Unsigned 8-byte (32-bit) integer
	U8,
	/// Signed 8-byte (32-bit) integer
	I8,
	/// 8-byte (32-bit) float
	F8,
	/// Unsigned X-byte (register-sized) integer
	UX,
	/// Signed X-byte (register-sized) integer
	IX,
	/// Boolean
	B,
	/// UTF-8 character
	C,
	/// Tuple
	T(Vec<DataType>),
	/// Function pointer
	F,
	/// Object pointer
	P
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
	UX(usize),
	IX(isize),
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

#[derive(Clone)]
enum Instruction {}

#[derive(Clone)]
enum Const {}

//--> Functions <--

impl Runtime {
	pub fn new() -> Runtime { Runtime { hooks: Vec::new(), code: Vec::new(), heap: Vec::new(), stack: Vec::new() } }

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

		let mut toks: HashMap<&Path, TokenStream> = HashMap::new();

		if let Some(os_ext) = path.extension() {
			if let Some(ext) = os_ext.to_str() {
				match ext {
					"rg" => {
						// source code
						match Token::lex_file(path) {
							Ok(t) => {
								toks.insert(path, t);
							},
							Err(mut e) => {
								errs.append(&mut e);
							}
						}
					},
					"robj" => {
						// bytecode
						// TODO
					},
					_ => {
						wrns.push(Error::new(format!("[WARN {}] File extension not recognized! Runtime will have to guess what kind of file this is.", path.display()), false, ErrorInfo::Load(LoadErr::InvalidInput)));
						// TODO
					}
				}
			} else {
				wrns.push(Error::new(format!("[WARN {}] File extension is not valid UTF-8! Runtime will have to guess what kind of file this is.", path.display()), false, ErrorInfo::Load(LoadErr::InvalidInput)));
				// TODO
			}
		} else {
			wrns.push(Error::new(format!("[WARN {}] No file extension! Runtime will have to guess what kind of file this is.", path.display()), false, ErrorInfo::Load(LoadErr::InvalidInput)));
			// TODO
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

		let mut toks: HashMap<&Path, TokenStream> = HashMap::new();

		for path in paths {
			match Token::lex_file(path) {
				Ok(t) => {
					toks.insert(path, t);
				},
				Err(mut e) => {
					errs.append(&mut e);
				}
			}
		}

		let actual_outfile = outfile.with_extension("robj").as_path();

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

		let mut toks: HashMap<&Path, TokenStream> = HashMap::new();

		for path in paths {
			match Token::lex_file(path) {
				Ok(t) => {
					toks.insert(path, t);
				},
				Err(mut e) => {
					errs.append(&mut e);
				}
			}
		}

		let actual_outfile = outfile.with_extension("robj").as_path();

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