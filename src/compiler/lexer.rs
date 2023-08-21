//--> Imports <--

use super::{
	super::{
		Error,
		ErrorKind,
		ErrorList,
	},
	InterpretError,
};

use std::{
	path::Path,
	fs::File,
	io::Read,
	str::FromStr,
	iter::Enumerate,
	vec::IntoIter,
};

use logos::{
	Lexer,
	Logos,
	Span
};

use nom::{
	InputIter,
};

//--> Type Aliases <--

pub(crate) type Result = std::result::Result<TokenStream, ErrorList>;

//--> Structs <--

/// Wrapper around a token, providing the character span of the token.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Token {
	pub inner: TokenInner,
	pub span: Span,
	pub slice: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct TokenStream(pub Vec<Token>);

//--> Enums <--

/// Tokens!
#[derive(Logos, Clone, Debug, PartialEq)]
pub(crate) enum TokenInner {
	/// A character literal is surrounded in single quotes and evaluates to a single Unicode character.
	#[regex(r"'(?:[^']|\\')'", TokenInner::lit_char)]
	LitChar(char),
	/// A string literal is surrounded in double quotes and evaluates to a, well, string of Unicode characters.
	/// _Raw_ string literals, which do not process escape sequences, are also surrounded with `r#`.
	#[regex(r#""(?:[^"]|\\")*""#, TokenInner::lit_char_str)]
	#[regex(r##"r#"?:[^("#r)]*"#r"##, TokenInner::lit_char_str_raw)]
	LitCharStr(String),
	/// A byte literal is surrounded in single quotes and `b` and evaluates to a single byte value.
	#[regex(r"b'(?:[^']|\\')'b", TokenInner::lit_byte)]
	LitByte(u8),
	/// A byte string literal is surrounded in double quotes and `b` and evaluates to a string of byte values.
	/// _Raw_ byte string literals, which do not process escape sequences, are also surrounded with `r#`.
	#[regex(r#"b"(?:[^("|("b))]|\\")*"b"#, TokenInner::lit_byte_str)]
	#[regex(r##"br#"?:[^("#rb)]*"#rb"##, TokenInner::lit_byte_str_raw)]
	LitByteStr(Vec<u8>),
	/// A general number literal. Can be in binary, octal, decimal, or hexadecimal.
	#[regex(r"0[bB][01][_01]*", TokenInner::bin)]
	#[regex(r"0[oO][0-7][_0-7]*", TokenInner::oct)]
	#[regex(r"[0-9][_0-9]*", TokenInner::dec)]
	#[regex(r"0[xX][0-9a-fA-F][_0-9a-fA-F]*", TokenInner::hex)]
	LitNum(u64),
	/// A floating point number literal. Must be in decimal.
	#[regex(r"[0-9][_0-9]*(.[0-9][_0-9]*|[eE][+-][0-9][_0-9]*|.[0-9][_0-9]*[eE][+-][0-9][_0-9]*)", TokenInner::float)]
	LitFloat(f64),
	/// Used for mutable re-assignment (binary) and variable declaration with explicit type
	#[token("=")]
	SymEqual,
	/// Used for equality checks (binary)
	#[token("==")]
	SymDEqual,
	/// Used for addition (binary) and identity (unary)
	#[token("+")]
	SymPlus,
	/// Used for mutable addition re-assignment (binary)
	#[token("+=")]
	SymPlusEqual,
	/// Used for subtraction (binary) and negation (unary)
	#[token("-")]
	SymDash,
	/// Used for mutable subtraction re-assignment (binary)
	#[token("-=")]
	SymDashEqual,
	/// Used for multiplication (binary)
	#[token("*")]
	SymStar,
	/// Used for mutable multiplication re-assignment (binary)
	#[token("*=")]
	SymStarEqual,
	/// Used for division (binary)
	#[token("/")]
	SymSlash,
	/// Used for mutable division re-assignment (binary)
	#[token("/=")]
	SymSlashEqual,
	/// Used for remainder (binary)
	#[token("%")]
	SymPercent,
	/// Used for divisibility checks (binary)
	#[token("%%")]
	SymDPercent,
	/// Used for mutable remainder re-assignment (binary)
	#[token("%=")]
	SymPercentEqual,
	/// Used for loop and block labels (unary)
	#[token("`")]
	SymBacktick,
	/// Used for bitwise or logical negation (unary) and for indicating calls to function-like macros
	#[token("!")]
	SymBang,
	/// Used for inequality checks (binary)
	#[token("!=")]
	SymBangEqual,
	/// Used for attributes/decorators (unary)
	#[token("@")]
	SymDecorator,
	/// Used for bitwise exclusive or (binary)
	#[token("^")]
	SymCaret,
	/// Used for mutable bitwise exclusive or re-assignment (binary)
	#[token("^=")]
	SymCaretEqual,
	/// Used for logical exclusive or (binary)
	#[token("^^")]
	SymDCaret,
	/// Used for bitwise and (binary)
	#[token("&")]
	SymAmpersand,
	/// Used for mutable bitwise and re-assignment (binary)
	#[token("&=")]
	SymAmpersandEqual,
	/// Used for logical and (binary)
	#[token("&&")]
	SymDAmpersand,
	/// Used for bitwise or (binary)
	#[token("|")]
	SymPipe,
	/// Used for bitwise or re-assignment (binary)
	#[token("|=")]
	SymPipeEqual,
	/// Used for logical or (binary)
	#[token("||")]
	SymDPipe,
	/// Used for array literals (`[val; num]`) and types (`[T; N]`), and putting multiple statements on one line
	#[token(";")]
	SymSemicolon,
	/// Used for static type labels (`val: str`), and map literals (`[key: val]`) and types (`[K: V]`)
	#[token(":")]
	SymColon,
	/// Used for variable declaration with inferred type
	#[token(":=")]
	SymWalrus,
	/// Used for namespace access
	#[token("::")]
	SymQuad,
	/// Used for separating collection members, function parameters, and type fields on one line
	#[token(",")]
	SymComma,
	/// Used for member accesses
	#[token(".")]
	SymDot,
	/// Used for method access and reassignment
	#[token(".=")]
	SymDotEqual,
	/// Used for monadic error propagation
	#[token("?")]
	SymTry,
	/// Used for monadic error propagation in a one-line method chain
	#[token("?.")]
	SymTryChain,
	/// Used for complex types, function parameters, and code blocks
	#[token("(")]
	SymOParen,
	/// Used for complex types, function parameters, and code blocks
	#[token(")")]
	SymCParen,
	/// Used for collections
	#[token("[")]
	SymOBracket,
	/// Used for collections
	#[token("]")]
	SymCBracket,
	/// Used for code blocks
	#[token("{")]
	SymOBrace,
	/// Used for code blocks
	#[token("}")]
	SymCBrace,
	/// Used for generic types, and less than checks (binary)
	#[token("<")]
	SymOAngle,
	/// Used for less than or equal checks (binary)
	#[token("<=")]
	SymLessEqual,
	/// Used for shift left (binary)
	#[token("<<")]
	SymDOAngle,
	/// Used for mutable shift left re-assignment (binary)
	#[token("<<=")]
	SymDOAngleEqual,
	/// Used for generic types, and greater than checks (binary)
	#[token(">")]
	SymCAngle,
	/// Used for function return types
	#[token("->")]
	SymThinArrow,
	/// Used for function effect types
	#[token("-<")]
	SymWeirdArrow,
	/// Used for greater than or equal checks (binary)
	#[token(">=")]
	SymGreaterEqual,
	/// Used for shift right (binary)
	#[token(">>")]
	SymDCAngle,
	/// Used for mutable shift right re-assignment (binary)
	#[token(">>=")]
	SymDCAngleEqual,
	/// Used for monadic bind
	#[token(">=>")]
	SymBind,
	/// Sometimes means the same thing as ; or ,
	#[token("\n")]
	SymNewline,
	/// Used for arbitrary operators (still considering whether I actually want this capability in Rouge)
	#[regex(r"\p{Punctuation}*", |l| l.slice().to_string())]
	SymUser(String),
	/// Type representing boolean logic values
	#[token("bool")]
	WordBoolType,
	/// Represents honest boolean logic values
	#[token("true")]
	WordTrue,
	/// Represents deceitful boolean logic values
	#[token("false")]
	WordFalse,
	/// Type representing binary data (byte::MIN = 0, byte::MAX = 255)
	#[token("byte")]
	WordByteType,
	/// Type representing unsigned whole numbers (nat::MIN = 0, nat::MAX = 18_446_744_073_709_551_615)
	#[token("nat")]
	WordNatType,
	/// Type representing signed whole numbers (int::MIN = -9_223_372_036_854_775_808, int::MAX = 9_223_372_036_854_775_807)
	#[token("int")]
	WordIntType,
	/// Type representing signed real numbers (flo::MIN = -1.797693134862315e+308, flo::MAX = 1.797693134862315e+308)
	#[token("flo")]
	WordFloType,
	/// Type representing any valid UTF-8 character
	#[token("char")]
	WordCharType,
	/// Type representing a string of valid UTF-8 characters
	#[token("str")]
	WordStrType,
	/// Represents the instance a method is being called on
	#[token("self")]
	WordSelf,
	/// Type of the implementer of a trait or method
	#[token("Self")]
	WordSelfType,
	/// Used to declare new types or type aliases
	#[token("type")]
	WordType,
	/// Used to declare new functions or function aliases
	#[token("func")]
	WordFunc,
	/// Used to declare a new trait
	#[token("trait")]
	WordTrait,
	/// Used to declare a new effect
	#[token("effect")]
	WordEffect,
	/// Used to implement a trait, or additional items in general, on a type
	#[token("impl")]
	WordImpl,
	/// Marks something as public - accessible to code in other packages or outside the runtime
	#[token("pub")]
	WordPub,
	/// Marks something as protected - by default, accessible to code within the same package only
	#[token("prt")]
	WordPrt,
	/// Marks a mutable variable
	#[token("mut")]
	WordMut,
	/// Marks a value which must be known at compile-time, or a function which can be used in a constant context
	#[token("const")]
	WordConst,
	/// Marks something which comes from outside the runtime, like a function or type
	#[token("extern")]
	WordExtern,
	/// Used for the first branch of a conditional block
	#[token("if")]
	WordIf,
	/// Used for the n-th branch of a conditional block
	#[token("elif")]
	WordElif,
	/// Used for pattern matching
	#[token("matches")]
	WordMatches,
	/// Used to separate conditions and patterns from the code that runs on true/match
	#[token("then")]
	WordThen,
	/// Used for the final branch of a conditional block
	#[token("else")]
	WordElse,
	/// Indicates a simple loop block.
	#[token("loop")]
	WordLoop,
	/// Indicates a conditional loop block, which loops while a condition is true.
	/// The condition is checked at the beginning of each iteration, meaning it might not run at all.
	#[token("while")]
	WordWhile,
	/// Indicates a conditional loop block, which loops until a condition is true.
	/// The condition is checked at the end of each iteration, meaning it will run at least once.
	#[token("until")]
	WordUntil,
	/// Indicates a for loop block, which loops over elements of an iterator.
	/// Desugars to an effect handler for `Yield::yield`.
	/// 
	/// Also used to indicate the type a trait is being implemented on.
	#[token("for")]
	WordFor,
	/// Indicates the iterator of a for loop.
	#[token("in")]
	WordIn,
	/// Indicates an effect handler.
	#[token("when")]
	WordWhen,
	/// Used in various places related to code blocks, such as:
	///  - at the end of function signatures `func func_name(arg: ArgType) -> ReturnType -< EffectType do`
	///  - at the end of while/until/for loop signatures `while/until condition do` or `for item in iter do`
	///  - at the end of effect handler signatures `when Effect::operation(args) do code`
	///  - between the arguments and code of a closure `(args) do code`
	#[token("do")]
	WordDo,
	/// Used between the signature and contents of a type declaration `type Name is fields`.
	#[token("is")]
	WordIs,
	/// Ends a block.
	#[token("end")]
	WordEnd,
	/// Indicates an imported package, module, item
	#[token("use")]
	WordUse,
	/// Used either to create import aliases OR as part of implementation blocks.
	#[token("as")]
	WordAs,
	/// Represents the parent module
	#[token("super")]
	WordSuper,
	/// Represents the package root
	#[token("pkg")]
	WordPkg,
	/// Alternative logical and operator
	#[token("and")]
	WordAnd,
	/// Alternative logical or operator
	#[token("or")]
	WordOr,
	/// Alternative logical exclusive or operator
	#[token("xor")]
	WordXor,
	/// Any legal identifier for a type, function, effect, trait, whatever
	#[regex(r"\p{XID_Start}\p{XID_Continue}*", |l| l.slice().to_string())]
	WordIdentifier(String),
	/// The obligatory error variant.
	#[error]
	#[regex(r"[ \t\r\f]+", logos::skip)]
	Error,
}

/// Errors that can occur while lexing.
#[derive(Clone, Debug)]
pub enum LexError {
	/// The token is invalid, plain and simple.
	/// 
	/// NOTE: Due to limitations in Logos, this is the only variant that will see any use for the time being.
	/// Logos, the crate being used to implement the lexer, doesn't really let you put data in the Error variant of your Token enum.
	/// This limits how good the error reporting can be, and I would like to look into getting this situation improved.
	InvalidToken,
	/// The lexer tried to parse this text as a number literal, but failed.
	NumberParseFail,
	/// The lexer tried to parse this text as a character, byte, string, or byte string literal, but it encountered an escape sequence it didn't recognize.
	UnknownEscapeSequence,
	/// The lexer tried to parse this text as a character, byte, string, or byte string literal, but it encountered a malformed ASCII/Byte or Unicode escape sequence.
	InvalidEscapeSequence,
}

//--> Functions <--

impl InputIter for TokenStream {
	type Item = Token;
	type Iter = Enumerate<Self::IterElem>;
	type IterElem = IntoIter<Token>;

	fn iter_indices(&self) -> Self::Iter {
		self.0.clone().into_iter().enumerate()
	}

	fn iter_elements(&self) -> Self::IterElem {
		self.0.clone().into_iter()
	}

	fn position<P>(&self, predicate: P) -> Option<usize>
		where
			P: Fn(Self::Item) -> bool {
		self.iter_indices().find(|(_, token)| predicate(token.clone())).map(|(pos, _)| pos)
	}

	fn slice_index(&self, count: usize) -> std::result::Result<usize, nom::Needed> {
		todo!()
	}
}

impl TokenStream {
	/// Given a path to a file of source code, converts it into a stream of tokens.
	/// This can fail, and the lexer will provide errors if so.
	pub(crate) fn lex_file(p: &Path) -> Result {
		match File::open(p) {
			Ok(mut f) => {
				let mut source = String::new();
				match f.read_to_string(&mut source) {
					Ok(_) => {
						let mut toks = Vec::new();

						for (token, span) in TokenInner::lexer(&source).spanned() {
							toks.push(Token {
								slice: (&source[span.clone()]).into(),
								span: span,
								inner: token,
							})
						}

						Ok(TokenStream(toks))
					},
					Err(e) => Err(vec![
						Error::new(false, Some(p), None, None, None, ErrorKind::IO(e.kind()))
					])
				}
			},
			Err(e) => Err(vec![
				Error::new(false, Some(p), None, None, None, ErrorKind::IO(e.kind()))
			])
		}
	}
}

impl TokenInner {
	/// Processes a character literal into an actual character.
	fn lit_char(l: &mut Lexer<TokenInner>) -> Option<char> {
		let slice: Vec<char> = l.slice().strip_prefix("'")?.strip_suffix("'")?.chars().collect();

		// Character literals cannot be empty.
		if slice.is_empty() { return None }

		// Character literals can only have one unicode character in them, UNLESS it contains an escape sequence.
		if slice.len() > 1 && slice[0] != '\\' { return None }

		if slice[0] == '\\' {
			match slice[1] {
				'0' => if slice.len() == 2 { Some('\0') } else { None },
				'a' => if slice.len() == 2 { char::from_u32(0x07) } else { None },
				'b' => if slice.len() == 2 { char::from_u32(0x08) } else { None },
				't' => if slice.len() == 2 { Some('\t') } else { None },
				'n' => if slice.len() == 2 { Some('\n') } else { None },
				'v' => if slice.len() == 2 { char::from_u32(0x0B) } else { None },
				'f' => if slice.len() == 2 { char::from_u32(0x0C) } else { None },
				'r' => if slice.len() == 2 { Some('\r') } else { None },
				'e' => if slice.len() == 2 { char::from_u32(0x1B) } else { None },
				'x' => if slice.len() == 4 {
					// ASCII escape sequence '\xFF' where FF is two hex digits
					let escape: String = (&slice[2..4]).iter().collect();
					
					let val = u8::from_str_radix(&escape, 16).ok()?;

					if val.is_ascii() { char::from_u32(val as u32) } else { None }
				} else { None },
				'u' => if slice.len() > 2 && slice[2] == '{' && slice[slice.len() - 1] == '}' {
					// Unicode escape sequence '\u{F*}' where F is a hex digit
					let escape: String = (&slice[3..slice.len() - 2]).iter().collect();

					char::from_u32(u32::from_str_radix(&escape, 16).ok()?)
				} else { None },
				c => if slice.len() == 2 { Some(c) } else { None }
			}
		} else {
			Some(slice[0])
		}
	}

	/// Processes a string literal into an actual string.
	fn lit_char_str(l: &mut Lexer<TokenInner>) -> Option<String> {
		let mut chars: Vec<char> = l.slice().strip_prefix('"')?.strip_suffix('"')?.chars().collect();

		let mut result = String::new();

		loop {
			let c0 = chars.pop()?;

			if c0 == '\\' {
				match chars.pop()? {
					'0' => result.push('\0'),
					'a' => result.push(char::from_u32(0x07)?),
					'b' => result.push(char::from_u32(0x08)?),
					't' => result.push('\t'),
					'n' => result.push('\n'),
					'v' => result.push(char::from_u32(0x0B)?),
					'f' => result.push(char::from_u32(0x0C)?),
					'r' => result.push('\r'),
					'e' => result.push(char::from_u32(0x1B)?),
					'x' => {
						// ASCII escape sequence '\xFF' where FF is two hex digits
						let mut escape = String::new();
						escape.push(chars.pop()?);
						escape.push(chars.pop()?);

						let val = u8::from_str_radix(&escape, 16).ok()?;

						if val.is_ascii() { result.push(char::from_u32(val as u32)?); } else { return None }
					},
					'u' => if chars.pop()? == '{' {
						// Unicode escape sequence '\u{F*}' where F is a hex digit
						let mut escape = String::new();

						loop {
							let cx = chars.pop()?;
							if cx == '}' { break; } else { escape.push(cx); }
						}

						result.push(char::from_u32(u32::from_str_radix(&escape, 16).ok()?)?);
					} else { return None },
					c => result.push(c)
				}
			} else {
				result.push(c0);
			}

			if chars.is_empty() { break }
		}

		Some(result)
	}

	/// Processes a raw string literal into an actual string.
	fn lit_char_str_raw(l: &mut Lexer<TokenInner>) -> Option<String> {
		Some(
			l.slice()
				.strip_prefix("r#\"")?
				.strip_suffix("\"#r")?
				.to_string()
		)
	}

	/// Processes a byte literal into an actual byte.
	fn lit_byte(l: &mut Lexer<TokenInner>) -> Option<u8> {
		let slice: Vec<char> = l.slice().strip_prefix("b'")?.strip_suffix("'b")?.chars().collect();

		// Byte literals cannot be empty.
		if slice.is_empty() { return None }

		// Byte literals cannot contain Unicode.
		if !slice.iter().all(|ch| ch.is_ascii()) { return None }

		// Byte literals can only have one ASCII character in them, UNLESS it contains an escape sequence.
		if slice.len() > 1 && slice[0] != '\\' { return None }

		if slice[0] == '\\' {
			match slice[1] {
				'0' => if slice.len() == 2 { Some(0x00) } else { None },
				'a' => if slice.len() == 2 { Some(0x07) } else { None },
				'b' => if slice.len() == 2 { Some(0x08) } else { None },
				't' => if slice.len() == 2 { Some(0x09) } else { None },
				'n' => if slice.len() == 2 { Some(0x0A) } else { None },
				'v' => if slice.len() == 2 { Some(0x0B) } else { None },
				'f' => if slice.len() == 2 { Some(0x0C) } else { None },
				'r' => if slice.len() == 2 { Some(0x0D) } else { None },
				'e' => if slice.len() == 2 { Some(0x1B) } else { None },
				'x' => if slice.len() == 4 {
					// ASCII escape sequence '\xFF' where FF is two hex digits
					let escape: String = (&slice[2..4]).iter().collect();
					
					u8::from_str_radix(&escape, 16).ok()
				} else { None },
				c => if slice.len() == 2 {
					let mut buf: [u8; 1] = [0];
					c.encode_utf8(&mut buf);
					Some(buf[0])
				} else { None }
			}
		} else {
			let mut buf: [u8; 1] = [0];
			slice[0].encode_utf8(&mut buf);
			Some(buf[0])
		}
	}

	/// Processes a byte string literal into an actual byte string.
	fn lit_byte_str(l: &mut Lexer<TokenInner>) -> Option<Vec<u8>> {
		let mut chars: Vec<char> = l.slice().strip_prefix("b\"")?.strip_suffix("\"b")?.chars().collect();

		// Byte string literals cannot contain Unicode.
		if !chars.iter().all(|ch| ch.is_ascii()) { return None }

		let mut result = Vec::new();
		let mut buf: [u8; 1] = [0];

		loop {
			let c0 = chars.pop()?;

			if c0 == '\\' {
				match chars.pop()? {
					'0' => result.push(0x00),
					'a' => result.push(0x07),
					'b' => result.push(0x08),
					't' => result.push(0x09),
					'n' => result.push(0x0A),
					'v' => result.push(0x0B),
					'f' => result.push(0x0C),
					'r' => result.push(0x0D),
					'e' => result.push(0x1B),
					'x' => {
						// ASCII escape sequence '\xFF' where FF is two hex digits
						let mut escape = String::new();
						escape.push(chars.pop()?);
						escape.push(chars.pop()?);

						result.push(u8::from_str_radix(&escape, 16).ok()?);
					},
					c => {
						c.encode_utf8(&mut buf);
						result.push(buf[0]);
					}
				}
			} else {
				c0.encode_utf8(&mut buf);
				result.push(buf[0]);
			}

			if chars.is_empty() { break }
		}

		Some(result)
	}

	/// Processes a raw byte string literal into an actual byte string.
	fn lit_byte_str_raw(l: &mut Lexer<TokenInner>) -> Option<Vec<u8>> {
		let chars: Vec<char> = l.slice().strip_prefix("br#\"")?.strip_suffix("\"#rb")?.chars().collect();

		// Raw byte string literals cannot contain Unicode.
		if !chars.iter().all(|ch| ch.is_ascii()) { return None }

		let mut buf: [u8; 1] = [0];

		Some(
			chars.iter()
				.map(|ch| {
					ch.encode_utf8(&mut buf);
					buf[0]
				})
				.collect()
		)
	}

	fn bin(l: &mut Lexer<TokenInner>) -> Option<u64> {
		u64::from_str_radix(
			l.slice()
				.to_lowercase()
				.strip_prefix("0b")?,
				2
		).ok()
	}

	fn oct(l: &mut Lexer<TokenInner>) -> Option<u64> {
		u64::from_str_radix(
			l.slice()
				.to_lowercase()
				.strip_prefix("0o")?,
				8
		).ok()
	}

	fn dec(l: &mut Lexer<TokenInner>) -> Option<u64> { u64::from_str(l.slice()).ok() }

	fn hex(l: &mut Lexer<TokenInner>) -> Option<u64> {
		u64::from_str_radix(
			l.slice()
				.to_lowercase()
				.strip_prefix("0x")?,
				16
		).ok()
	}

	fn float(l: &mut Lexer<TokenInner>) -> Option<f64> { f64::from_str(l.slice()).ok() }
}

//--> Unit Testing <--

#[cfg(test)]
mod tests {
	use super::*;
}