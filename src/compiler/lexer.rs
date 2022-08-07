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
	path::{
		Path,
	},
	fs::File,
	io::{
		BufRead,
		BufReader,
	},
	str::FromStr,
};

use logos::{
	Filter,
	Lexer,
	Logos,
	Span
};

//--> Type Aliases <--

pub(crate) type TokenStream = Vec<TokenWrapper>;

pub(crate) type Result = std::result::Result<TokenStream, ErrorList>;

//--> Structs <--

/// Wrapper around a token, providing the character span of the token.
#[derive(Clone)]
pub(crate) struct TokenWrapper {
	pub inner: Token,
	pub span: Span,
	pub slice: String,
}

//--> Enums <--

/// Tokens!
#[derive(Logos, Clone)]
pub(crate) enum Token {
	
	/// Literal values.
	/// The `\u{0}-\u{10FFFE}\u{10FFFF}` in the UTF character and string regexes is there to hopefully circumvent a bug in Logos.
	/// This bug apparently causes `\u{0}-\u{10FFFF}` to match _any byte_ rather than any Unicode character.
	#[regex(r"'(\\'|[\u{0}-\u{10FFFE}\u{10FFFF}]+)'", Lit::utf_char)]
	#[regex(r#""([\u{0}-\u{10FFFE}\u{10FFFF}]|\\")*""#, Lit::utf_str)]
	#[regex(r##"r#"[\u{0}-\u{10FFFE}\u{10FFFF}]*"#r"##, Lit::raw_utf_str)]
	#[regex(r"b'(\\'|[\x00-\x7F]+)'b", Lit::byte_char)]
	#[regex(r#"b"([\x00-\x7F]|\\")*"b"#, Lit::byte_str)]
	#[regex(r##"br#"[\x00-\x7F]*"#rb"##, Lit::raw_byte_str)]
	#[regex(r"[+-]?0(B|b)[01][_01]*", Lit::bin)]
	#[regex(r"[+-]?0(O|o)[0-7][_0-7]*", Lit::oct)]
	#[regex(r"[+-]?[0-9][_0-9]*(.[0-9][_0-9]+)?((E|e)[0-9][_0-9]+)?", Lit::dec)]
	#[regex(r"[+-]?0(X|x)[0-9a-fA-F][_0-9a-fA-F]*", Lit::hex)]
	Literal(Lit),

	/// Symbols and special characters.
	#[regex(r"[\p{Punctuation}\n]", Op::new)]
	Operator(Op),

	/// Keywords... and identifiers.
	#[regex(r"\p{XID_Start}\p{XID_Continue}*", Word::new)]
	Keyword(Word),
	
	///
	#[error]
	#[regex(r"[ \t\r\f]+", logos::skip)]
	Error,
}

/// Literal values
#[derive(Clone)]
pub(crate) enum Lit {
	
	/// UTF-8 character literal, 'x'
	UTFCharacter(char),
	
	/// UTF-8 string literal, "x" or r#"x"#r
	UTFString(String),
	
	/// Byte character literal, b'x'b
	ByteCharacter(u8),

	/// Byte string literal, b"x" or br#"x"#rb
	ByteString(Vec<u8>),
	
	/// Unsigned integer, in decimal/hexadecimal/octal/binary
	UnsignedInteger(u64),
	
	/// Signed integer, in decimal/hexadecimal/octal/binary
	SignedInteger(i64),
	
	/// Floating point number, in decimal.
	Float(f64),
}

/// Operators (symbols, special characters)
#[derive(Clone)]
pub(crate) enum Op {
	/// The backtick symbol is used to start a loop label.
	Tick,
	/// The bang or exclamation mark symbol is used to mark macro calls.
	/// It can also be used for bitwise and logical not operations.
	/// The bang and equals symbols combine to perform not-equal comparison `!=`.
	Bang,
	/// The at symbol is used to start decorator/attribute lines.
	Decorator,
	/// The hash, number, or pound symbol is used to start comment lines.
	Comment,
	/// The percent symbol is used to perform remainder operations (divide, return the remainder).
	Remainder,
	/// The caret symbol is used to perform bitwise exlcusive or operations.
	BitwiseExclusiveOr,
	/// The ampersand symbol is used to perform bitwise and operations.
	/// Two in a row can be used to perform logical and operations, which short-circuit.
	BitwiseAnd,
	/// The multiply symbol is used to perform multiplication operations.
	/// Two in a row performs exponent operations.
	Multiply,
	/// Opening parentheses are used to start tuples or the arguments to a function or any macros.
	OpenParentheses,
	/// Closing parentheses are used to close tuples or the arguments to a function or any macros.
	CloseParentheses,
	/// The minus symbol is used to perform subtraction operations, or negate a value.
	Minus,
	/// The equals symbol is used for variable assignment.
	/// Two in a row performs equal comparison, and other comparison operators also use the equals symbol.
	Equals,
	/// The plus symbol is used to perform addition operations.
	Plus,
	/// Opening square brackets are used to start collections, as well as groups of items imported from a module.
	OpenSquare,
	/// Closing square brackets are used to close collections, as well as groups of items imported from a module.
	CloseSquare,
	/// Opening curly brackets are used to start structure initializations.
	OpenCurly,
	/// Closing curly brackets are used to close structure initializations.
	CloseCurly,
	/// The pipe symbol is used to perform bitwise or operations.
	/// Two in a row can be used to perform logical or operations, which short-circuit.
	BitwiseOr,
	/// The semicolon symbol is used to separate multiple statements on the same line.
	Semicolon,
	/// The colon symbol is used to mark the start of a multi-line code block, or separate the keys and values of a map.
	/// Two in a row accesses descendant items (not including methods).
	Colon,
	/// The comma symbol is used to separate most things.
	Comma,
	/// The dot, full stop, or period symbol is used to access members of a tuple, struct, or enum, or invoke methods on a struct or enum.
	Dot,
	/// Opening angle brackets are used to start generic type parameters.
	/// Two in a row performs bitwise shift left operations.
	/// One can also be used to perform less-than comparison, and can be combined with equals to perform less-or-equal comparison `<=`.
	OpenAngle,
	/// Closing angle brackets are used to close generic type parameters.
	/// Two in a row performs bitwise shift right operations.
	/// One can also be used to perform greater-than comparison, and can be combined with equals to perform greater-or-equal comparison `>=`.
	CloseAngle,
	/// The slash symbol is used to perform division operations.
	Divide,
	/// The question mark symbol is used to perform error propagation on types implementing the `Try` trait.
	Try,
	/// Indicates a new line. Self-explanatory, really.
	Newline,
}

/// Keywords and identifiers
#[derive(Clone)]
pub(crate) enum Word {
	/// `byte` keyword used to represent the type of an unsigned 8-bit integer (aka a byte)
	Byte,
	/// `nat` keyword used to represent the type of an unsigned 64-bit integer (aka natural number)
	Natural,
	/// `int` keyword used to represent the type of a signed 64-bit integer
	Integer,
	/// `float` keyword used to represent the type of a 64-bit floating-point number
	Float,
	/// `bool` keyword used to represent the type of a true or false value
	Boolean,
	/// `true` keyword that represents a true or positive boolean value
	True,
	/// `false` keyword that represents a false or negative boolean value
	False,
	/// `char` keyword used to represent the type of a UTF-8 character
	Character,
	/// `string` keyword used to represent the type of a list (or string) of characters (basically a primitive/built-in type alias)
	String,
	/// `var` keyword used to invoke type inferrence on a variable (i.e. when the type should be obvious).
	/// 
	/// Can ONLY be used with variables. Use anywhere else will generate a parsing error.
	VariableType,
	/// `self` or `Self` keyword used to represent either:
	/// 
	///  - The package containing the module you're working on
	///  - The type of an implementor of a trait
	///  - The instance of a custom type that a method was called/invoked on
	Selff,
	/// `super` keyword used to represent the parent module of the module you're working on
	Super,
	/// `mut` keyword that marks a value as mutable, able to be changed
	Mutable,
	/// `const` keyword that marks a value as constant, needing to be known before any code runs and unable to be changed.
	/// It also marks functions that don't have side-effects.
	/// Constant functions must have a return value and must not take mutable arguments. In addition, they cannot call non-constant functions.
	Constant,
	/// `func` keyword that starts a function
	Function,
	/// `struct` keyword that starts a structure definition
	Structure,
	/// `enum` keyword that starts an enumeration definition
	Enumeration,
	/// `trait` keyword that starts a trait definition
	Trait,
	/// `impl` keyword that starts a block to implement things onto a structure or enumerationnnnnn
	Implementation,
	/// `pub` keyword that marks something as public, directly accessible by any code outside of the package or even the runtime
	Public,
	/// `prt` keyword that marks something as protected, only accessible within a certain bound. This bound is, by default, the package
	Protected,
	/// An `if` expression takes a condition (returns a bool) and runs code if the condition is met.
	If,
	/// `if x is y` == Rust's `if let y = x`
	Is,
	/// `elif` extends an `if` expression, adding another 'arm' that takes another condition. If all preceeding conditions are false, this 'arm' runs its code if its condition is met.
	ElseIf,
	/// `else` acts as the final 'arm' of an `if` or `match` expression.
	Else,
	/// The `do` keyword serves multiple purposes.
	/// 
	/// First, it can be used to start a generic code block. For example: (Rust equivalent in comments)
	/// ```
	/// var x = do:			#	let x = {
	/// 	outl!("Hello!")	#		println!("Hello!");
	/// 	return 42		#		42
	/// end					#	};
	/// ```
	/// 
	/// Second, it can be used alongside two pipe characters to create a _closure_ - an anonymous function, essentially.
	/// ```
	/// var sum = |x, y| do x + y
	/// 
	/// # Closures can use variables from the context they are defined in.
	/// var fruit = "Banana"
	/// var g = || do return fruit[0] # returns 'B'
	/// 
	/// # Closures can also be multi-line.
	/// var repeat = |times| do:
	/// 	return fruit.repeated_with(times, ' ')
	/// end
	/// ```
	/// 
	/// Finally, it is used to end the signatures of while, until, and for loop expressions.
	/// ```
	/// for i in 1..=100 do outl!("{}", i)
	/// ```
	Do,
	/// `then` separates the condition from the code that should be run if that condition is met.
	/// 
	/// ## `if`/`elif` example:
	/// ```
	/// if condition then code() elif condition then other_code()
	/// ```
	Then,
	/// `loop` creates a loop that runs forever without intervention.
	Loop,
	/// `while` creates a loop that runs while the given condition is met, and checks that condition at the start of each iteration.
	While,
	/// `until` creates a loop that runs until the given condition is met, and checks that condition at the end of each iteration.
	Until,
	/// `for` either creates a loop that runs once for each member of a collection, or indicates the type a trait is being implemented for.
	For,
	/// `in` specifies the collection used in a for loop.
	In,
	/// `skip` skips the current iteration of a loop.
	Skip,
	/// `break` breaks out of a loop, continuing on with the rest of the program.
	Break,
	/// `end` marks the end of a multi-line code block.
	End,
	/// The `return` keyword is used to explicitly return a value from a control flow expression or function, or to return a value from an iteration of a loop while simultaneously ending the loop like the `break` keyword.
	Return,
	/// The `yield` keyword is used to return a value from an iteration of a loop while simultaneously skipping to the next iteration like the `skip` keyword.
	/// In the future, when async/await gets added, `yield` will be used to return a value from a co-routine while not ending the co-routine entirely.
	Yield,
	/// `extern` marks runtime hooks - functions or types that come from the runtime or program embedding the runtime.
	External,
	/// `use` is used to import modules, types, and functions from other packages and/or modules.
	Use,
	/// `as` has two main uses:
	/// 
	///  - To create a local alias for something that's being imported using `use`
	///  - To perform an unsafe (might panic) cast between primitive types
	As,
	/// `and` is used to perform logical and operations, which short-circuit. If a false value is encountered, nothing further will be checked.
	LogicalAnd,
	/// `or` is used to perform logical or operations, which short-circuit. If a true value is encountered, nothing further will be checked.
	LogicalOr,
	/// `xor` is used to perform logical exclusive or operations.
	LogicalExclusiveOr,
	/// `not` can be used to perform logical or bitwise not operations.
	Not,
	/// A catch-all for anything caught as a keyword that isn't actually a keyword (i.e. identifiers)
	Identifier(String),
}

/// Errors that can occur while lexing.
#[derive(Clone)]
#[repr(u8)]
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

impl Token {
	/// Given a path to a file of source code, converts it into a stream of tokens.
	/// This can fail, and the lexer will provide errors if so.
	pub(crate) fn lex_file(p: &Path) -> Result {
		let mut toks = Vec::new();
		let mut errs = Vec::new();
		
		match File::open(p) {
			Ok(f) => {
				for (lno, line) in BufReader::new(f).lines().enumerate() {
					match line {
						Ok(l) => {
							for (token, span) in Token::lexer(&l).spanned() {
								match token {
									// TODO: Better error messages. Really don't know how I could do this without Logos allowing arguments in the error variant.
									Token::Error => errs.push(Error::new(false, Some(p), Some(lno), Some(span.clone()), Some(&l[span]), ErrorKind::Interpret(InterpretError::Lex(LexError::InvalidToken)))),
									_ => toks.push(TokenWrapper { inner: token, span: span.clone(), slice: String::from(&l[span]) })
								}
							}
						},
						Err(e) => errs.push(Error::new(false, Some(p), Some(lno), None, None, ErrorKind::IO(e.kind())))
					}
				}
			},
			Err(e) => errs.push(Error::new(false, Some(p), None, None, None, ErrorKind::IO(e.kind())))
		}

		if errs.is_empty() {
			Ok(toks)
		} else {
			Err(errs)
		}
	}

	/// Given a line of text provided by the user over stdin, converts it into a stream of tokens.
	/// This can fail, and the lexer will provide errors if so.
	pub(crate) fn lex_line(s: &str, lno: usize) -> Result {
		let mut toks = Vec::new();
		let mut errs = Vec::new();

		for (token, span) in Token::lexer(s).spanned() {
			match token {
				// TODO: Better error messages. Really don't know how I could do this without Logos allowing arguments in the error variant.
				Token::Error => errs.push(Error::new(false, None, Some(lno), Some(span.clone()), Some(&s[span]), ErrorKind::Interpret(InterpretError::Lex(LexError::InvalidToken)))),
				_ => toks.push(TokenWrapper { inner: token, span: span.clone(), slice: String::from(&s[span]) })
			}
		}

		if errs.is_empty() {
			Ok(toks)
		} else {
			Err(errs)
		}
	}
}

impl Lit {
	/// Callback function to construct a Unicode character from a character literal.
	pub fn utf_char(l: &mut Lexer<Token>) -> Option<Lit> {
		// Stripping the delimiting apostrophes so we don't have to worry about dealing with them.
		// Turning this into a character list so we can index individual characters, rather than the underlying bytes.
		let chars = l.slice().strip_prefix("'")?.strip_suffix("'")?.chars().collect::<Vec<char>>();

		// An empty character literal is an error. (If only Logos let us provide extra error information, le sigh)
		if !chars.is_empty() {
			if chars[0] == '\\' {
				// A backslash starts an escape sequence, which must have at least one more character.
				if chars.len() > 1 {
					match chars[1] {
						'\'' => Some(Lit::UTFCharacter('\'')),
						'\\' => Some(Lit::UTFCharacter('\\')),
						'0' => Some(Lit::UTFCharacter('\0')),
						't' => Some(Lit::UTFCharacter('\t')),
						'r' => Some(Lit::UTFCharacter('\r')),
						'n' => Some(Lit::UTFCharacter('\n')),
						'x' => {
							// ASCII escape sequence, \xFF where FF is any 2-digit hexadecimal number, 4 characters total
							if chars.len() == 4 {
								let mut valstr = String::new();

								valstr.push(chars[2]);
								valstr.push(chars[3]);

								if let Ok(v) = u8::from_str_radix(&valstr, 16) {
									if v.is_ascii() {
										Some(Lit::UTFCharacter(char::from_u32(v as u32)?))
									} else { None }
								} else { None }
							} else { None }
						},
						'u' => {
							// Unicode escape sequence, \u{F} where F is a hexadecimal number with 1 to 6 digits, 5 to 10 characters total
							if chars.len() >= 5 && chars.len() <= 10 {
								if chars[2] == '{' && chars[chars.len() - 1] == '}' {
									let mut valstr = String::new();

									for cx in 3..(chars.len() - 1) {
										valstr.push(chars[cx]);
									}

									if let Ok(v) = u32::from_str_radix(&valstr, 16) {
										Some(Lit::UTFCharacter(char::from_u32(v)?))
									} else { None }
								} else { None }
							} else { None }
						},
						_ => None // Unrecognized escape sequence!
					}
				} else { None }
			} else {
				// Normal character. Ensure there's just one character - any more is an error.
				if chars.len() == 1 {
					Some(Lit::UTFCharacter(chars[0]))
				} else { None }
			}
		} else { None }
	}

	/// Callback function to construct a Unicode string from a string literal.
	pub fn utf_str(l: &mut Lexer<Token>) -> Option<Lit> {
		// Stripping the delimiting quotes so we don't have to deal with them.
		// Also, using this as a stack so we need to reverse it before collecting the characters into a Vec.
		let mut chars = l.slice().strip_prefix('"')?.strip_suffix('"')?.chars().rev().collect::<Vec<char>>();

		if !chars.is_empty() {
			let mut return_string = String::new();

			loop {
				// I know it's guaranteed to yield a character but better safe then sorry.
				let c0 = chars.pop()?;

				if c0 == '\\' {
					match chars.pop()? {
						'"' => return_string.push('"'),
						'\\' => return_string.push('\\'),
						'0' => return_string.push('\0'),
						't' => return_string.push('\t'),
						'r' => return_string.push('\r'),
						'n' => return_string.push('\n'),
						'x' => {
							// ASCII escape sequence, \xFF where FF is any 2-digit hexadecimal number, 4 characters total
							let mut valstr = String::new();

							valstr.push(chars.pop()?);
							valstr.push(chars.pop()?);

							if let Ok(v) = u8::from_str_radix(&valstr, 16) {
								if v.is_ascii() {
									valstr.push(char::from_u32(v as u32)?);
								} else { return None }
							} else { return None }
						},
						'u' => {
							// Unicode escape sequence, \u{F} where F is a hexadecimal number with 1 to 6 digits, 5 to 10 characters total
							if chars.pop()? == '{' {
								let mut valstr = String::new();

								loop {
									let cx = chars.pop()?;
									if cx == '}' {
										break;
									} else {
										valstr.push(cx);
									}
								}

								if let Ok(v) = u32::from_str_radix(&valstr, 16) {
									valstr.push(char::from_u32(v)?);
								} else { return None }
							} else { return None }
						},
						_ => return None // Unrecognized escape sequence!
					}
				} else {
					return_string.push(c0);
				}

				if chars.is_empty() {
					break;
				}
			}

			Some(Lit::UTFString(return_string))
		} else {
			// Empty strings are allowed.
			Some(Lit::UTFString(String::new()))
		}
	}

	/// Callback function to construct a Unicode string from a raw string literal.
	pub fn raw_utf_str(l: &mut Lexer<Token>) -> Option<Lit> {
		// Since raw strings don't process escape sequences, we just need to strip the delimiters and we're basically done.
		let slice = l.slice().strip_prefix("r#\"")?.strip_suffix("\"#r")?;

		Some(Lit::UTFString(String::from(slice)))
	}

	fn char_to_byte(c: char) -> Option<u8> {
		// Would be nicer if the encode_utf8 function just returned a slice but whatever...
		let mut buf: [u8; 4] = [0x00; 4];
		c.encode_utf8(&mut buf);
		// If any bytes other than the first one are non-zero, we have a non-ASCII character which isn't allowed.
		if buf[1] == 0x00 && buf[2] == 0x00 && buf[3] == 0x00 {
			Some(buf[0])
		} else { None }
	}

	/// Callback function to construct a byte from a byte character literal.
	pub fn byte_char(l: &mut Lexer<Token>) -> Option<Lit> {
		// Stripping the delimiting apostrophes and b's so we don't have to worry about dealing with them.
		// Turning this into a character list so we can index individual characters, rather than the underlying bytes.
		let chars = l.slice().strip_prefix("b'")?.strip_suffix("'b")?.chars().collect::<Vec<char>>();

		// An empty character literal is an error. (If only Logos let us provide extra error information, le sigh)
		if !chars.is_empty() {
			if chars[0] == '\\' {
				// A backslash starts an escape sequence, which must have at least one more character.
				if chars.len() > 1 {
					match chars[1] {
						'\'' => Some(Lit::ByteCharacter(0x27)),
						'\\' => Some(Lit::ByteCharacter(0x5C)),
						'0' => Some(Lit::ByteCharacter(0x00)),
						't' => Some(Lit::ByteCharacter(0x09)),
						'r' => Some(Lit::ByteCharacter(0x0D)),
						'n' => Some(Lit::ByteCharacter(0x0A)),
						'x' => {
							// Byte escape sequence, \xFF where FF is any 2-digit hexadecimal number, 4 characters total
							if chars.len() == 4 {
								let mut valstr = String::new();

								valstr.push(chars[2]);
								valstr.push(chars[3]);

								if let Ok(v) = u8::from_str_radix(&valstr, 16) {
									Some(Lit::ByteCharacter(v))
								} else { None }
							} else { None }
						},
						_ => None // Unrecognized escape sequence!
					}
				} else { None }
			} else {
				// Normal character. Ensure there's just one character - any more is an error.
				if chars.len() == 1 {
					Some(Lit::ByteCharacter(Lit::char_to_byte(chars[0])?))
				} else { None }
			}
		} else { None }
	}

	/// Callback function to construct a byte string from a byte string literal.
	pub fn byte_str(l: &mut Lexer<Token>) -> Option<Lit> {
		// Stripping the delimiting quotes so we don't have to deal with them.
		// Also, using this as a stack so we need to reverse it before collecting the characters into a Vec.
		let mut chars = l.slice().strip_prefix('"')?.strip_suffix('"')?.chars().rev().collect::<Vec<char>>();

		if !chars.is_empty() {
			let mut return_string = Vec::new();

			loop {
				let c0 = chars.pop()?;

				if c0 == '\\' {
					match chars.pop()? {
						'"' => return_string.push(0x22),
						'\\' => return_string.push(0x5C),
						'0' => return_string.push(0x00),
						't' => return_string.push(0x09),
						'r' => return_string.push(0x0D),
						'n' => return_string.push(0x0A),
						'x' => {
							// Byte escape sequence, \xFF where FF is any 2-digit hexadecimal number, 4 characters total
							let mut valstr = String::new();

							valstr.push(chars.pop()?);
							valstr.push(chars.pop()?);

							if let Ok(v) = u8::from_str_radix(&valstr, 16) {
								return_string.push(v);
							} else { return None }
						},
						_ => return None
					}
				} else {
					return_string.push(Lit::char_to_byte(c0)?);
				}

				if chars.is_empty() {
					break;
				}
			}

			Some(Lit::ByteString(return_string))
		} else {
			// Empty strings are allowed.
			Some(Lit::ByteString(Vec::new()))
		}
	}

	/// Callback function to construct a byte string from a raw byte string literal.
	pub fn raw_byte_str(l: &mut Lexer<Token>) -> Option<Lit> {
		let chars = l.slice().strip_prefix("br#\"")?.strip_suffix("\"#rb")?.chars().collect::<Vec<char>>();
		let mut return_string = Vec::new();

		for cx in chars {
			return_string.push(Lit::char_to_byte(cx)?);
		}

		Some(Lit::ByteString(return_string))
	}

	/// Callback function to parse a binary number literal.
	pub fn bin(l: &mut Lexer<Token>) -> Option<Lit> {
		let mut slice = l.slice();

		let is_negative = slice.starts_with('-');
		let is_signed = slice.starts_with('+') || is_negative;

		if slice.starts_with('+') {
			slice = slice.strip_prefix('+').unwrap();
		} else if slice.starts_with('-') {
			slice = slice.strip_prefix('-').unwrap();
		}

		if slice.starts_with("0B") {
			slice = slice.strip_prefix("0B").unwrap();
		} else if slice.starts_with("0b") {
			slice = slice.strip_prefix("0b").unwrap();
		} else { return None }

		if is_signed {
			let mut num = i64::from_str_radix(slice, 2).ok()?;

			if is_negative {
				num = -num;
			}

			Some(Lit::SignedInteger(num))
		} else {
			Some(Lit::UnsignedInteger(u64::from_str_radix(slice, 2).ok()?))
		}
	}

	/// Callback function to parse an octal number literal.
	pub fn oct(l: &mut Lexer<Token>) -> Option<Lit> {
		let mut slice = l.slice();

		let is_negative = slice.starts_with('-');
		let is_signed = slice.starts_with('+') || is_negative;

		if slice.starts_with('+') {
			slice = slice.strip_prefix('+').unwrap();
		} else if slice.starts_with('-') {
			slice = slice.strip_prefix('-').unwrap();
		}

		if slice.starts_with("0O") {
			slice = slice.strip_prefix("0O").unwrap();
		} else if slice.starts_with("0o") {
			slice = slice.strip_prefix("0o").unwrap();
		} else { return None }

		if is_signed {
			let mut num = i64::from_str_radix(slice, 8).ok()?;

			if is_negative {
				num = -num;
			}

			Some(Lit::SignedInteger(num))
		} else {
			Some(Lit::UnsignedInteger(u64::from_str_radix(slice, 8).ok()?))
		}
	}

	/// Callback function to parse a decimal number literal.
	pub fn dec(l: &mut Lexer<Token>) -> Option<Lit> {
		let slice = l.slice();

		if slice.contains('.') || slice.contains('e') || slice.contains("E") {
			// Float
			Some(Lit::Float(f64::from_str(slice).ok()?))
		} else if slice.starts_with('+') || slice.starts_with('-') {
			// Signed integer
			Some(Lit::SignedInteger(i64::from_str(slice).ok()?))
		} else {
			// Unsigned integer
			Some(Lit::UnsignedInteger(u64::from_str(slice).ok()?))
		}
	}

	/// Callback function to parse a hexadecimal number literal.
	pub fn hex(l: &mut Lexer<Token>) -> Option<Lit> {
		let mut slice = l.slice();

		let is_negative = slice.starts_with('-');
		let is_signed = slice.starts_with('+') || is_negative;

		if slice.starts_with('+') {
			slice = slice.strip_prefix('+').unwrap();
		} else if slice.starts_with('-') {
			slice = slice.strip_prefix('-').unwrap();
		}

		if slice.starts_with("0X") {
			slice = slice.strip_prefix("0X").unwrap();
		} else if slice.starts_with("0x") {
			slice = slice.strip_prefix("0x").unwrap();
		} else { return None }

		if is_signed {
			let mut num = i64::from_str_radix(slice, 16).ok()?;

			if is_negative {
				num = -num;
			}

			Some(Lit::SignedInteger(num))
		} else {
			Some(Lit::UnsignedInteger(u64::from_str_radix(slice, 16).ok()?))
		}
	}
}

impl Op {
	/// Callback function to recognize special characters as operators.
	pub fn new(l: &mut Lexer<Token>) -> Filter<Op> {
		match l.slice() {
			"`" => Filter::Emit(Op::Tick),
			"!" => Filter::Emit(Op::Bang),
			"@" => Filter::Emit(Op::Decorator),
			"#" => Filter::Emit(Op::Comment),
			"%" => Filter::Emit(Op::Remainder),
			"^" => Filter::Emit(Op::BitwiseExclusiveOr),
			"&" => Filter::Emit(Op::BitwiseAnd),
			"*" => Filter::Emit(Op::Multiply),
			"(" => Filter::Emit(Op::OpenParentheses),
			")" => Filter::Emit(Op::CloseParentheses),
			"-" => Filter::Emit(Op::Minus),
			"=" => Filter::Emit(Op::Equals),
			"+" => Filter::Emit(Op::Plus),
			"[" => Filter::Emit(Op::OpenSquare),
			"]" => Filter::Emit(Op::CloseSquare),
			"{" => Filter::Emit(Op::OpenCurly),
			"}" => Filter::Emit(Op::CloseCurly),
			"|" => Filter::Emit(Op::BitwiseOr),
			";" => Filter::Emit(Op::Semicolon),
			":" => Filter::Emit(Op::Colon),
			"," => Filter::Emit(Op::Comma),
			"." => Filter::Emit(Op::Dot),
			"<" => Filter::Emit(Op::OpenAngle),
			">" => Filter::Emit(Op::CloseAngle),
			"/" => Filter::Emit(Op::Divide),
			"?" => Filter::Emit(Op::Try),
			"\n" => Filter::Emit(Op::Newline),
			_ => Filter::Skip
		}
	}
}

impl Word {
	/// Callback function to recognize keywords and custom identifiers.
	pub fn new(l: &mut Lexer<Token>) -> Word {
		let slice = l.slice();

		match slice {
			"byte" => Word::Byte,
			"nat" => Word::Natural,
			"int" => Word::Integer,
			"float" => Word::Float,
			"bool" => Word::Boolean,
			"true" => Word::True,
			"false" => Word::False,
			"char" => Word::Character,
			"string" => Word::String,
			"var" => Word::VariableType,
			"self" | "Self" => Word::Selff,
			"super" => Word::Super,
			"mut" => Word::Mutable,
			"const" => Word::Constant,
			"func" => Word::Function,
			"struct" => Word::Structure,
			"enum" => Word::Enumeration,
			"trait" => Word::Trait,
			"impl" => Word::Implementation,
			"pub" => Word::Public,
			"prt" => Word::Protected,
			"if" => Word::If,
			"is" => Word::Is,
			"elif" => Word::ElseIf,
			"else" => Word::Else,
			"do" => Word::Do,
			"then" => Word::Then,
			"loop" => Word::Loop,
			"while" => Word::While,
			"until" => Word::Until,
			"for" => Word::For,
			"in" => Word::In,
			"skip" => Word::Skip,
			"break" => Word::Break,
			"end" => Word::End,
			"return" => Word::Return,
			"yield" => Word::Yield,
			"extern" => Word::External,
			"use" => Word::Use,
			"as" => Word::As,
			"and" => Word::LogicalAnd,
			"or" => Word::LogicalExclusiveOr,
			"xor" => Word::LogicalOr,
			"not" => Word::Not,
			_ => Word::Identifier(String::from(slice))
		}
	}
}