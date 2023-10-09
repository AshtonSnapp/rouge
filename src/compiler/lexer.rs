//--> Imports <--

use std::{
	cmp::Ordering,
	collections::HashMap,
	convert::From,
	error::Error as ErrorTrait,
	fmt,
	fs::File,
	io,
	io::{
		Error as IoError,
		ErrorKind as IoErrorKind,
		Read,
		Result as IoResult,
		Stdin,
	},
	iter::Iterator,
	ops::RangeInclusive,
	path::Path,
	str,
};

use read_chars::ReadChars;

use thiserror::Error;

//--> Type Aliases <--

pub type Result = std::result::Result<Token, Error>;

pub type TokenSpan = RangeInclusive<TokenPos>;

//--> Structs <--

/// Recognizes Rouge tokens within text read from the given source.
pub(crate) struct Lexer<R: Read> {
	source: ReadChars<R>,
	current_line: usize,
	current_char: usize,
}

#[derive(Clone, Debug)]
pub struct TokenPos {
	pub line: usize,
	pub col: usize,
}

/// Contains information about a token of source code.
pub struct Token {
	/// 
	pub kind: TokenKind,
	pub source: String,
	pub span: TokenSpan,

}

/// Contains information about an error encountered while trying to lex a token.
#[derive(Debug, Error)]
pub struct Error {
	///
	pub kind: ErrorKind,
	///
	pub source: Option<Box<dyn ErrorTrait>>,
	///
	pub source_text: String,
	///
	pub span: TokenSpan,
}

//--> Enums <--

///
pub enum TokenKind {
	/// A UTF-8 character literal.
	LitChar(char),
	/// A UTF-8 string literal.
	LitStr(String),
	/// An interpolated UTF-8 string literal.
	LitStrInterpSegment(String, SegmentType),
	/// A byte character literal.
	LitByte(u8),
	/// A byte string literal.
	LitByteStr(Vec<u8>),
	/// A 64-bit unsigned integer literal. Can be binary, octal, decimal, or hexadecimal.
	LitNum(u64),
	/// A 64-bit floating-point literal. Can be decimal or hexadecimal.
	LitFlo(f64),
}

///
pub enum SegmentType {
	/// The first segment of an interpolated UTF-8 string literal. Will look something like /"*\\{/
	Start,
	/// A middle segment of an interpolated UTF-8 string literal. Will look something like /}*\\{/
	Middle,
	/// The last segment of an interpolated UTF-8 string literal. Will look something like /}*"/
	End,
}

///
#[derive(Debug)]
pub enum ErrorKind {
	/// An IO error occured while trying to lex a token. Please check the source field to see what the error is.
	Io,
	/// You tried to create a character literal, but it either is empty or contains a single unescaped backslash.
	/// Alternatively, you tried to create an escape sequence that requires more characters than you gave.
	NotEnoughCharacters,
	/// You tried to create a character literal, but it contains more than one character.
	TooManyCharacters,
	/// You tried to create a text literal containing an escape sequence that wasn't recognized.
	InvalidEscapeSequence,
	/// You tried to create a text literal containing an ASCII escape representing an invalid character.
	InvalidAsciiEscape,
	/// You tried to create a text literal containing a Unicode escape representing an invalid character.
	InvalidUnicodeEscape,
}

//-> Functions <--

impl From<(usize, usize)> for TokenPos {
	fn from(value: (usize, usize)) -> Self {
		TokenPos {
			line: value.0,
			col: value.1,
		}
	}
}

impl<R: Read> Lexer<R> {
	/// Borrows a string slice and returns a Lexer over the bytes of that slice.
	/// 
	/// The Read trait is not implemented for `&str`, but it is for `&[u8]`.
	/// Because of this, the string slice must be converted into a byte slice.
	pub fn lex_str(str: &str) -> Lexer<&[u8]> {
		Lexer {
			source: ReadChars::from(str.as_bytes()),
			current_line: 1,
			current_char: 0,
		}
	}

	/// Takes in the path to a file and returns a Lexer over that file, if no error occurs.
	pub fn lex_file(path: &Path) -> IoResult<Lexer<File>> {
		Ok(Lexer {
			source: ReadChars::from(File::open(path)?),
			current_line: 1,
			current_char: 0,
		})
	}

	/// Returns a lexer over the standard input stream.
	pub fn lex_stdin() -> Lexer<Stdin> {
		Lexer {
			source: ReadChars::from(io::stdin()),
			current_line: 1,
			current_char: 0,
		}
	}
}

impl<R: Read> Iterator for Lexer<R> {
	type Item = Result;

	fn next(&mut self) -> Option<Self::Item> {
		let chr0 = match self.source.next()? {
			Ok(c) => c,
			Err(e) => return Some(Err(Error {
				kind: ErrorKind::Io,
				source: Some(Box::new(e)),
				source_text: "".into(),
				span: {
					let start = TokenPos { line: self.current_line, col: self.current_char };
					let end = TokenPos { line: self.current_line, col: self.current_line };

					start..=end
				},
			}))
		};

		todo!()
	}
}

impl<R: Read> Lexer<R> {
	fn lit_char(source: &str, span: TokenSpan) -> Result {
		// Unwrapping here because the single quotes around the literal
		// would have been verified before the main function passes the
		// source text over here.
		let chars = source.strip_prefix('\'').unwrap()
			.strip_suffix('\'').unwrap()
			.chars().collect::<Vec<_>>();

		if chars.is_empty() {
			return Err(Error {
				kind: ErrorKind::NotEnoughCharacters,
				source: None,
				source_text: source.into(),
				span,
			});
		}
		
		if chars[0] == '\\' {
			let mut escape_sequence = String::from("\\");
			let mut escape_sequence_span: TokenSpan = TokenPos { line: span.start().line, col: span.start().col + 1, }..=TokenPos { line: span.end().line, col: span.end().line - 1 };

			if chars.len() >= 2 {
				match chars[1] {
					'0' => Ok(Token {
						kind: TokenKind::LitChar('\0'),
						source: source.into(),
						span,
					}),
					'\\' => Ok(Token {
						kind: TokenKind::LitChar('\\'),
						source: source.into(),
						span,
					}),
					'\'' => Ok(Token {
						kind: TokenKind::LitChar('\''),
						source: source.into(),
						span,
					}),
					'n' => Ok(Token {
						kind: TokenKind::LitChar('\n'),
						source: source.into(),
						span,
					}),
					'r' => Ok(Token {
						kind: TokenKind::LitChar('\r'),
						source: source.into(),
						span,
					}),
					't' => Ok(Token {
						kind: TokenKind::LitChar('\t'),
						source: source.into(),
						span,
					}),
					'x' => {
						escape_sequence.push('x');
						escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: escape_sequence_span.start().col + 1, ..*escape_sequence_span.end() };

						match chars.len().cmp(&4usize) {
							Ordering::Equal => {
								let val: String = chars[2..=3].iter().collect();
								escape_sequence.push_str(&val);
								escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: escape_sequence_span.start().col + 2, ..*escape_sequence_span.end() };

								match u8::from_str_radix(&val, 16) {
									Ok(num) => if num.is_ascii() {
										match char::from_u32(num as u32) {
											Some(chr) => Ok(Token {
												kind: TokenKind::LitChar(chr),
												source: source.into(),
												span,
											}),
											None => Err(Error {
												kind: ErrorKind::InvalidAsciiEscape,
												source: None,
												source_text: escape_sequence,
												span: escape_sequence_span,
											}),
										}
									} else {
										Err(Error {
											kind: ErrorKind::InvalidAsciiEscape,
											source: None,
											source_text: escape_sequence,
											span: escape_sequence_span,
										})
									},
									Err(e) => Err(Error {
										kind: ErrorKind::InvalidAsciiEscape,
										source: Some(Box::new(e)),
										source_text: escape_sequence,
										span: escape_sequence_span,
									}),
								}
							},
							Ordering::Less => Err(Error {
								kind: ErrorKind::NotEnoughCharacters,
								source: None,
								source_text: if chars.len() > 2 { escape_sequence.push(chars[2]); escape_sequence } else { escape_sequence },
								span: if chars.len() > 2 { *escape_sequence_span.start()..=TokenPos { col: escape_sequence_span.end().col + 1, ..*escape_sequence_span.end() } } else { escape_sequence_span },
							}),
							Ordering::Greater => {
								escape_sequence.push_str(&chars[2..].iter().collect::<String>());
								escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: span.end() - 1, ..*escape_sequence_span.end() };

								Err(Error {
									kind: ErrorKind::TooManyCharacters,
									source: None,
									source_text: escape_sequence,
									span: escape_sequence_span,
								})
							},
						}
					},
					'u' => {
						escape_sequence.push('u');
						escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: escape_sequence_span.start().col + 1, ..*escape_sequence_span.end() };

						if chars.len() >= 5 {
							escape_sequence.push_str(&chars[2..chars.len() - 1].iter().collect::<String>());
							escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: span.end().col - 1, ..*escape_sequence_span.end() };
							
							if chars[2] == '{' && chars[chars.len() - 1] == '}' {
								let val = chars[3..=chars.len() - 2].iter().collect::<String>();

								match u32::from_str_radix(&val, 16) {
									Ok(num) => match char::from_u32(num) {
										Some(chr) => Ok(Token {
											kind: TokenKind::LitChar(chr),
											source: source.into(),
											span,
										}),
										None => Err(Error {
											kind: ErrorKind::InvalidUnicodeEscape,
											source: None,
											source_text: escape_sequence,
											span: escape_sequence_span,
										})
									},
									Err(e) => Err(Error {
										kind: ErrorKind::InvalidUnicodeEscape,
										source: Some(Box::new(e)),
										source_text: escape_sequence,
										span: escape_sequence_span,
									})
								}
							} else {
								Err(Error {
									kind: ErrorKind::InvalidUnicodeEscape,
									source: None,
									source_text: escape_sequence,
									span: escape_sequence_span,
								})
							}
						} else {
							if chars.len() > 2 {
								escape_sequence.push_str(&chars[2..chars.len() - 1].iter().collect::<String>());
								escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: span.end().col - 1, ..*escape_sequence_span.end() };
							}
							
							Err(Error {
								kind: ErrorKind::NotEnoughCharacters,
								source: None,
								source_text: escape_sequence,
								span: escape_sequence_span,
							})
						}
					},
					char => {
						escape_sequence.push(char);
						escape_sequence_span = *escape_sequence_span.start()..=TokenPos { col: escape_sequence_span.start().col + 1, ..*escape_sequence_span.end() };

						Err(Error {
							kind: ErrorKind::InvalidEscapeSequence,
							source: None,
							source_text: escape_sequence,
							span: escape_sequence_span,
						})
					},
				}
			} else {
				Err(Error {
					kind: ErrorKind::NotEnoughCharacters,
					source: None,
					source_text: escape_sequence,
					span: escape_sequence_span,
				})
			}
		} else {
			Ok(Token {
				kind: TokenKind::LitChar(chars[0]),
				source: source.into(),
				span,
			})
		}
	}

	fn lit_str(source: &str, span: TokenSpan) -> Result {
		let mut chars = source.strip_prefix('"').unwrap()
			.strip_suffix('"').unwrap()
			.chars().collect::<Vec<_>>();

		let mut buffer = String::new();

		if !chars.is_empty() {
			let current_line: usize = span.start().line;
			let current_col: usize = span.start().col;
			
			loop {
				let c0 = chars.pop().unwrap();

				if c0 == '\\' {
					let mut escape_sequence: String = "\\".into();
					let mut escape_span: TokenSpan = TokenPos { line: current_line, col: current_col }..=TokenPos { line: current_line, col: current_col };

					
				} else {
					buffer.push(c0);
				}
			}
		}

		Ok(Token {
			kind: TokenKind::LitStr(buffer),
			source: source.into(),
			span,
		})
	}

	fn lit_str_interp_segment(source: &str, span: TokenSpan) -> Result {
		todo!()
	}
	
	fn lit_str_raw(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn lit_byte(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn lit_byte_str(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn lit_byte_str_raw(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn binary(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn octal(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn decimal(source: &str, span: TokenSpan) -> Result {
		todo!()
	}

	fn hexadecimal(source: &str, span: TokenSpan) -> Result {
		todo!()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		todo!()
	}
}

//--> Unit Testing <--

#[cfg(test)]
mod tests {}