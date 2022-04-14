//--> Imports <--

use super::{Error, ErrorInfo, ErrorList, ParseErr};
use std::io::ErrorKind as LoadErr;
use std::path::Path;
use std::fs::File;
use logos::{Lexer, Logos, skip};
use std::str::FromStr;
use std::io::{BufRead, BufReader};

//--> Type Aliases <--

/// A stream of tokens.
pub type TokenStream = Vec<Token>;

/// A stream of tokens with warnings, or errors and warnings.
pub type TokenResult = Result<TokenStream, ErrorList>;

//--> Enums <--

#[derive(Logos, Clone)]
#[logos(subpattern bin = r"[01][_01]*")]
#[logos(subpattern oct = r"[0-7][_0-7]*")]
#[logos(subpattern dec = r"[0-9][_0-9]*")]
#[logos(subpattern hex = r"[0-9a-fA-F][_0-9a-fA-F]*")]
#[logos(subpattern exp = r"[eE][+-]?[0-9][_0-9]*")]
pub enum Token {
	#[regex(r"'(?:[^']|\\')*'", Lit::char)]
	#[regex(r#""(?:[^"]|\\")*""#, Lit::char_str)]
	#[regex(r##"r#"(?:[^"]|\\")*"#r"##, Lit::raw_char_str)]
	#[regex(r"b'(?:[^']|\\')*'b", Lit::byte)]
	#[regex(r#"b"(?:[^"]|\\")*"b"#, Lit::byte_str)]
	#[regex(r##"br#"(?:[^"]|\\")*"#rb"##, Lit::raw_byte_str)]
	#[regex(r"0[bB](?&bin)", Lit::bin)]
	#[regex(r"0[oO](?&oct)", Lit::oct)]
	#[regex(r"0[xX](?&hex)", Lit::hex)]
	#[regex(r#"[+-]?(((?&dec)\.(?&dec)?(?&exp)?)|((?&dec)(?&exp)?))"#, Lit::dec)]
	Literal(Lit),

	#[regex(r"[`!@%^&*()-=+\[]\{}|;:,.<>/?]", Op::new)]
	Operator(Op),

	#[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", Wrd::new)]
	Word(Wrd),

	Newline, // never constructed by Logos

	#[error]
	#[regex(r"[ \t\n\r\f]+", skip)]	// ignore whitespace
	#[regex(r"#[^\n]*", skip)] // ignore comments
	Error
}

#[derive(Clone)]
pub enum Lit {
	Char(char),
	CharStr(String),
	Byte(u8),
	ByteStr(Vec<u8>),
	UnsignedInt(u64),
	SignedInt(i64),
	Float(f64)
}

#[derive(Clone)]
pub enum Op {
	Tick,
	Bang,
	At,
	Modulo,
	Caret,
	Ampersand,
	Star,
	OpenParen,
	CloseParen,
	Dash,
	Equal,
	Plus,
	OpenSquare,
	CloseSquare,
	OpenCurly,
	CloseCurly,
	Pipe,
	Semicolon,
	Colon,
	Comma,
	Dot,
	OpenAngle,
	CloseAngle,
	Slash,
	Question
}

#[derive(Clone)]
pub enum Wrd {
	UnsignedByte,	// ubyte
	SignedByte,		// byte
	UnsignedShort,	// ushort
	SignedShort,	// short
	UnsignedWord,	// uword
	SignedWord,		// word
	Float,			// flt
	UnsignedLong,	// ulong
	SignedLong,		// long
	Double,			// dbl
	UnsignedInt,	// uint
	SignedInt,		// int
	Boolean,		// bool
	True,			// true
	False,			// false
	Character,		// char
	String,			// string
	Selff,			// self
	InferType,		// var
	Constant,		// const
	Function,		// func
	Closure,		// do
	TypeAlias,		// type
	Structure,		// struct
	Enumeration,	// enum
	Implement,		// impl
	Trait,			// trait
	Public,			// pub
	TraitObject,	// dyn
	Is,				// is
	If,				// if
	ElseIf,			// elif
	Else,			// else
	MatchBlock,		// match
	MatchArm,		// then
	Loop,			// loop
	While,			// while
	Until,			// until
	For,			// for
	In,				// in
	EndBlock,		// end
	LogicalNot,		// not
	LogicalAnd,		// and
	LogicalOr,		// or
	LogicalExclOr,	// xor
	Identifier(String)
}

//--> Functions <--

impl Token {
	pub fn lex_file(p: &Path) -> TokenResult {
		let mut toks = Vec::new();
		let mut errs = Vec::new();

		// Attempt to open the file.
		match File::open(p) {
			Ok(f) => {
				for (lno, line) in BufReader::new(f).lines().enumerate() {
					match line {
						Ok(l) => {
							for (tok, span) in Token::lexer(&l).spanned() {
								let s = span.clone();
								match tok {
									Token::Error => errs.push(Error::new(format!("<ERR! {}:{}:{}..{}> '{}' couldn't be turned into a token.", p.display(), lno, span.start, span.end, &l[s]), false, ErrorInfo::Parse(ParseErr::InvalidToken))),
									_ => toks.push(tok)
								}
							}
							toks.push(Token::Newline);
						},
						Err(e) => match e.kind() {
							// The only error I'm anticipating is invalid UTF-8 data.
							LoadErr::InvalidData => errs.push(Error::new(format!("<ERR! {}:{}> Encountered invalid UTF-8 while trying to read this line.", p.display(), lno), false, ErrorInfo::Load(e.kind()))),
							_ => errs.push(Error::new(format!("<ERR! {}:{}> Encountered unexpected IO error {:?} while trying to read this line.", p.display(), lno, e.kind()), false, ErrorInfo::Load(e.kind())))
						}
					}
				}
			},
			Err(e) => match e.kind() {
				// The only kinds of errors I'm anticipating here are the file not existing, and the file being inaccessible due to permissions.
				LoadErr::NotFound => errs.push(Error::new(format!("<ERR! {}> Couldn't find this file.", p.display()), false, ErrorInfo::Load(e.kind()))),
				LoadErr::PermissionDenied => errs.push(Error::new(format!("<ERR! {}> Denied permission to open this file.", p.display()), false, ErrorInfo::Load(e.kind()))),
				_ => errs.push(Error::new(format!("<ERR! {}> Encountered unexpected IO error '{:?}' while trying to open this file.", p.display(), e.kind()), false, ErrorInfo::Load(e.kind())))
			}
		}

		if errs.is_empty() {
			Ok(toks)
		} else {
			Err(errs)
		}
	}

	pub fn lex_line(s: &str, lno: usize) -> TokenResult {
		let mut toks = Vec::new();
		let mut errs = Vec::new();

		for (tok, span) in Token::lexer(s).spanned() {
			let sp = span.clone();
			match tok {
				Token::Error => errs.push(Error::new(format!("<ERR! stdin:{}:{}..{}> '{}' couldn't be turned into a token.", lno, span.start, span.end, &s[sp]), false, ErrorInfo::Parse(ParseErr::InvalidToken))),
				_ => toks.push(tok)
			}
		}
		toks.push(Token::Newline);

		if errs.is_empty() {
			Ok(toks)
		} else {
			Err(errs)
		}
	}
}

impl Lit {
	pub fn char(l: &mut Lexer<Token>) -> Option<Lit> {
		// Get a list of all the characters in the literal. Multiple characters is supported because escape sequences.
		let chars = l.slice().strip_prefix("'")?.strip_suffix("'")?.chars().collect::<Vec<char>>();

		if chars[0] == '\\' {
			// We have an escape sequence. Make sure we have at least two characters in the literal.
			if chars.len() >= 2 {
				match chars[1] {
					'\'' => Some(Lit::Char('\'')),
					'\\' => Some(Lit::Char('\\')),
					'0' => Some(Lit::Char('\0')), // null
					'n' => Some(Lit::Char('\n')), // line feed / *nix newline
					'r' => Some(Lit::Char('\r')), // carriage return / half of a Windows newline
					't' => Some(Lit::Char('\t')), // tab
					'x' => {
						// ASCII escape sequence. Make sure there's two more characters (four characters total) in the literal.
						if chars.len() == 4 {
							let mut v = String::new();
							v.push(chars[2]);
							v.push(chars[3]);

							if let Ok(n) = u8::from_str_radix(&v, 16) {
								// Make sure we're in ASCII range. (ASCII = 0x00 inclusive to 0x80 exclusive / 0 inclusive to 128 exclusive)
								if n < 128 {
									Some(Lit::Char(char::from_u32(n as u32)?))
								} else { None }
							} else { None }
						} else { None }
					},
					'u' => {
						// Unicode escape sequence. Make sure there's three to eight more characters (five to ten characters total) in the literal.
						if chars.len() >= 5 && chars.len() <= 10 {
							// Unicode escape format: \u{X} where X is 0 to 10FFFF
							if chars[2] == '{' {
								if chars[chars.len() - 1] == '}' {
									let mut v = String::new();
									for cx in chars[3..chars.len() - 2].iter() {
										v.push(*cx);
									}

									if let Ok(n) = u32::from_str_radix(&v, 16) {
										Some(Lit::Char(char::from_u32(n)?))
									} else { None }
								} else { None }
							} else { None }
						} else { None }
					},
					_ => None // invalid escape sequence
				}
			} else { None }
		} else {
			// We don't have an escape sequence. Make sure we only have one character in the literal.
			if chars.len() == 1 {
				Some(Lit::Char(chars[0]))
			} else { None }
		}
	}

	pub fn char_str(l: &mut Lexer<Token>) -> Option<Lit> {
		let mut ret = String::new();

		// character stack
		let mut chars = l.slice().strip_prefix("\"")?.strip_suffix("\"")?.chars().rev().collect::<Vec<char>>();

		// If the character stack is empty, we want to return early with an empty string.
		if chars.is_empty() {
			return Some(Lit::CharStr(ret))
		}

		loop {
			let c0 = chars.pop()?;

			if c0 == '\\' {
				match chars.pop()? {
					'"' => ret.push('"'),
					'\\' => ret.push('\\'),
					'0' => ret.push('\0'),
					'n' => ret.push('\n'),
					'r' => ret.push('\r'),
					't' => ret.push('\t'),
					'x' => {
						// ASCII escape sequence
						let mut v = String::new();
						v.push(chars.pop()?);
						v.push(chars.pop()?);

						if let Ok(n) = u8::from_str_radix(&v, 16) {
							if n < 128 {
								ret.push(char::from_u32(n as u32)?);
							} else { return None }
						} else { return None }
					},
					'u' => {
						// Unicode escape sequence
						let mut v = String::new();
						if chars.pop()? == '{' {
							loop {
								let cx = chars.pop()?;
								if cx == '}' { break; } else { v.push(cx); }
							}
							ret.push(char::from_u32(u32::from_str_radix(&v, 16).ok()?)?);
						} else { return None }
					},
					_ => return None
				}
			} else { ret.push(c0); }
			
			if chars.is_empty() { break; }
		}

		Some(Lit::CharStr(ret))
	}

	pub fn raw_char_str(l: &mut Lexer<Token>) -> Option<Lit> {
		let s = l.slice().strip_prefix("r#\"")?.strip_suffix("\"#r")?;
		Some(Lit::CharStr(String::from(s)))
	}

	pub fn byte(l: &mut Lexer<Token>) -> Option<Lit> {
		// Get a list of all the characters in the literal. Multiple characters is supported because escape sequences.
		let chars = l.slice().strip_prefix("b'")?.strip_suffix("'b")?.chars().collect::<Vec<char>>();

		if chars[0] == '\\' {
			// We have an escape sequence. Make sure we have at least two characters in the literal.
			if chars.len() >= 2 {
				match chars[1] {
					'\'' => Some(Lit::Byte(0x27)),
					'\\' => Some(Lit::Byte(0x5C)),
					'0' => Some(Lit::Byte(0x00)), // null
					'n' => Some(Lit::Byte(0x0A)), // line feed / *nix newline
					'r' => Some(Lit::Byte(0x0D)), // carriage return / half of a Windows newline
					't' => Some(Lit::Byte(0x09)), // horizontal tab
					'x' => {
						// Byte escape sequence. Make sure there's two more characters (four characters total) in the literal.
						if chars.len() == 4 {
							let mut v = String::new();
							v.push(chars[2]);
							v.push(chars[3]);

							Some(Lit::Byte(u8::from_str_radix(&v, 16).ok()?))
						} else { None }
					},
					_ => None
				}
			} else { None }
		} else {
			// We don't have an escape sequence. Make sure we only have one character in the literal, and that it is in ASCII range.
			if chars.len() == 1 {
				if chars[0].is_ascii() {
					// This is kinda stupid, but the encode_utf8() method doesn't just return an array or slice.
					let mut buf = [0x00];
					chars[0].encode_utf8(&mut buf);
					Some(Lit::Byte(buf[0]))
				} else { None }
			} else { None }
 		}
	}

	pub fn byte_str(l: &mut Lexer<Token>) -> Option<Lit> {
		let mut ret = Vec::new();

		// character stack
		let mut chars = l.slice().strip_prefix("b\"")?.strip_suffix("\"b")?.chars().rev().collect::<Vec<char>>();

		// If the character stack is empty, we'll want to return early with an empty byte list.
		if chars.is_empty() {
			return Some(Lit::ByteStr(ret))
		}

		loop {
			let c0 = chars.pop()?;

			if c0 == '\\' {
				match chars.pop()? {
					'"' => ret.push(0x22),
					'\\' => ret.push(0x5C),
					'0' => ret.push(0x00),
					'n' => ret.push(0x0A),
					'r' => ret.push(0x0D),
					't' => ret.push(0x09),
					'x' => {
						// byte escape sequence
						let mut v = String::new();
						v.push(chars.pop()?);
						v.push(chars.pop()?);

						ret.push(u8::from_str_radix(&v, 16).ok()?);
					},
					_ => return None
				}
			} else {
				if c0.is_ascii() {
					let mut buf = [0x00];
					c0.encode_utf8(&mut buf);
					ret.push(buf[0]);
				} else { return None }
			}
			
			if chars.is_empty() { break; }
		}

		Some(Lit::ByteStr(ret))
	}

	pub fn raw_byte_str(l: &mut Lexer<Token>) -> Option<Lit> {
		let mut ret = Vec::new();

		for c in l.slice().strip_prefix("br#\"")?.strip_suffix("\"#rb")?.chars() {
			if c.is_ascii() {
				let mut buf = [0x00];
				c.encode_utf8(&mut buf);
				ret.push(buf[0]);
			} else { return None }
		}

		Some(Lit::ByteStr(ret))
	}

	pub fn bin(l: &mut Lexer<Token>) -> Option<Lit> {
		// Get the slice and strip the prefix.
		let s = l.slice().to_lowercase();
		let s = s.strip_prefix("0b")?;

		// Turn the slice into an unsigned 64-bit integer.
		let n = u64::from_str_radix(s, 2).ok()?;

		// Return an unsigned integer literal token.
		Some(Lit::UnsignedInt(n))
	}

	pub fn oct(l: &mut Lexer<Token>) -> Option<Lit> {
		// Get the slice and strip the prefix.
		let s = l.slice().to_lowercase();
		let s = s.strip_prefix("0o")?;

		// Turn the slice into an unsigned 64-bit integer.
		let n = u64::from_str_radix(s, 8).ok()?;

		// Return an unsigned integer literal token.
		Some(Lit::UnsignedInt(n))
	}

	pub fn hex(l: &mut Lexer<Token>) -> Option<Lit> {
		// Get the slice and strip the prefix.
		let s = l.slice().to_lowercase();
		let s = s.strip_prefix("0x")?;

		// Turn the slice into an unsigned 64-bit integer.
		let n = u64::from_str_radix(s, 16).ok()?;

		// Return an unsigned integer literal token.
		Some(Lit::UnsignedInt(n))
	}

	pub fn dec(l: &mut Lexer<Token>) -> Option<Lit> {
		let s = l.slice().to_lowercase();
		let s = s.as_str();

		if s.contains(".") || s.contains("e") {
			// float
			let n = f64::from_str(s).ok()?;
			Some(Lit::Float(n))
		} else if s.starts_with("+") || s.starts_with("-") {
			// signed int
			let n = i64::from_str(s).ok()?;
			Some(Lit::SignedInt(n))
		} else {
			// unsigned int
			let n = u64::from_str(s).ok()?;
			Some(Lit::UnsignedInt(n))
		}
	}
}

impl Op {
	pub fn new(l: &mut Lexer<Token>) -> Option<Op> {
		match l.slice() {
			"`" => Some(Op::Tick),
			"!" => Some(Op::Bang),
			"@" => Some(Op::At),
			"%" => Some(Op::Modulo),
			"^" => Some(Op::Caret),
			"&" => Some(Op::Ampersand),
			"*" => Some(Op::Star),
			"(" => Some(Op::OpenParen),
			")" => Some(Op::CloseParen),
			"-" => Some(Op::Dash),
			"=" => Some(Op::Equal),
			"+" => Some(Op::Plus),
			"[" => Some(Op::OpenSquare),
			"]" => Some(Op::CloseSquare),
			"{" => Some(Op::OpenCurly),
			"}" => Some(Op::CloseCurly),
			"|" => Some(Op::Pipe),
			";" => Some(Op::Semicolon),
			":" => Some(Op::Colon),
			"," => Some(Op::Comma),
			"." => Some(Op::Dot),
			"<" => Some(Op::OpenAngle),
			">" => Some(Op::CloseAngle),
			"/" => Some(Op::Slash),
			"?" => Some(Op::Question),
			_ => None
		}
	}
}

impl Wrd {
	pub fn new(l: &mut Lexer<Token>) -> Wrd {
		let s = l.slice();

		match s {
			"ubyte" => Wrd::UnsignedByte,
			"byte" => Wrd::SignedByte,
			"ushort" => Wrd::UnsignedShort,
			"short" => Wrd::SignedShort,
			"uword" => Wrd::UnsignedWord,
			"word" => Wrd::SignedWord,
			"flt" => Wrd::Float,
			"ulong" => Wrd::UnsignedLong,
			"long" => Wrd::SignedLong,
			"dbl" => Wrd::Double,
			"uint" => Wrd::UnsignedInt,
			"int" => Wrd::SignedInt,
			"bool" => Wrd::Boolean,
			"true" => Wrd::True,
			"false" => Wrd::False,
			"char" => Wrd::Character,
			"string" => Wrd::String,
			"self" => Wrd::Selff,
			"var" => Wrd::InferType,
			"const" => Wrd::Constant,
			"func" => Wrd::Function,
			"do" => Wrd::Closure,
			"type" => Wrd::TypeAlias,
			"struct" => Wrd::Structure,
			"enum" => Wrd::Enumeration,
			"impl" => Wrd::Implement,
			"trait" => Wrd::Trait,
			"pub" => Wrd::Public,
			"dyn" => Wrd::TraitObject,
			"is" => Wrd::Is,
			"if" => Wrd::If,
			"elif" => Wrd::ElseIf,
			"else" => Wrd::Else,
			"match" => Wrd::MatchBlock,
			"then" => Wrd::MatchArm,
			"loop" => Wrd::Loop,
			"while" => Wrd::While,
			"until" => Wrd::Until,
			"for" => Wrd::For,
			"in" => Wrd::In,
			"end" => Wrd::EndBlock,
			"not" => Wrd::LogicalNot,
			"and" => Wrd::LogicalAnd,
			"or" => Wrd::LogicalOr,
			"xor" => Wrd::LogicalExclOr,
			_ => Wrd::Identifier(String::from(s))
		}
	}
}