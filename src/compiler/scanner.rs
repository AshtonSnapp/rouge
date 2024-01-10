//--> Imports & Modules <--

use std::{
    iter::Iterator,
    path::Path,
    str::FromStr, fs,
};

use logos::{
    Lexer,
    Logos,
    Span,
    SpannedIter,
};

//--> Type Aliases <--

//--> Structs <--

pub(crate) struct Scanner<'a> {
    inner: SpannedIter<'a, TokenKind>,
} 

pub struct Token {
    pub kind: TokenKind,
    pub code: String,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct StrInterpPart {
    pub string: String,
    pub kind: PartKind,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Error {
    span: Option<Span>,
}

//--> Enums <--

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
#[logos(error = Error)]
pub enum TokenKind {
    
    /// A UTF-8 character literal.
    #[regex(r"'(?:[^']|\\')'", callback = process_char)]
    LitChar(char),
    /// A UTF-8 string literal, either cooked or raw.
    #[regex(r#""(?:[^"]|\\")""#, callback = process_str)]
    #[regex(r##"r#"(?:[^("#r)])"#r"##, callback = |lex| lex.slice().strip_prefix("r#\"").unwrap().strip_suffix("\"#r").unwrap().to_string())]
    LitStr(String),
    /// Part of an interpolated UTF-8 string literal.
    #[regex(r#""((?:[^(\\\{|\\")])|(\\"))*(\\\{)"#, callback = process_str_interp)]
    #[regex(r#"\}(((?:[^(\\\{|\\")])|(\\"))*|(\\"))(\\\{)"#, callback = process_str_interp)]
    #[regex(r#"\}(((?:[^"])|(\\"))*|(\\"))""#, callback = process_str_interp)]
    LitStrInterpPart(StrInterpPart),
    /// A byte character literal.
    #[regex(r"b'([\x00-\x7F]+)|(\\')'b", callback = process_byte)]
    LitByte(u8),
    /// A byte string literal, either cooked or raw.
    #[regex(r#"b"(?:[^"]|\\")"b"#, callback = process_byte_str)]
    #[regex(r##"br#"(?:[^("#r)])"#rb"##, callback = process_raw_byte_str)]
    LitByteStr(Vec<u8>),
    /// A 64-bit number, stored as unsigned here.
    #[regex(r"0[bB][01]([01_]*)", callback = |lex| u64::from_str_radix(lex.slice().to_lowercase().strip_prefix("0b").unwrap(), 2).map_err(|e| Error {
        span: None,
    }))]
    #[regex(r"0[oO][0-7]([0-7_]*)", callback = |lex| u64::from_str_radix(lex.slice().to_lowercase().strip_prefix("0o").unwrap(), 8).map_err(|e| Error {
        span: None,
    }))]
    #[regex(r"[0-9]([0-9_]*)", callback = |lex| u64::from_str(lex.slice()).map_err(|e| Error {
        span: None,
    }))]
    #[regex(r"0[xX][0-9a-fA-F]([0-9a-fA-F]*)", callback = |lex| u64::from_str_radix(lex.slice().to_lowercase().strip_prefix("0x").unwrap(), 16).map_err(|e| Error {
        span: None,
    }))]
    LitNumber(u64),
    /// A 64-bit floating-point number.
    #[regex(r"[0-9]([0-9_]*)((.[0-9]([0-9_]*))|([eE][+-][0-9]([0-9_]*))|(.[0-9]([0-9_]*)[eE][+-][0-9]([0-9_]*)))", callback = |lex| f64::from_str(lex.slice()).map_err(|e| Error {
        span: None,
    }))]
    LitFloat(f64),

    ///
    #[token("+")]
    SymPlus,
    ///
    #[token("-")]
    SymDash,
    ///
    #[token("*")]
    SymStar,
    ///
    #[token("/")]
    SymSlash,
    ///
    #[token("%")]
    SymPercent,
    ///
    #[token("%%")]
    SymDoublePercent,
    ///
    #[token("&")]
    SymAnd,
    ///
    #[token("&&")]
    SymDoubleAnd,
    ///
    #[token("|")]
    SymPipe,
    ///
    #[token("||")]
    SymDoublePipe,
    ///
    #[token("^")]
    SymCaret,
    ///
    #[token("!")]
    SymBang,
    ///
    #[token(",")]
    SymComma,
    ///
    #[token(".")]
    SymDot,
    ///
    #[token(";")]
    SymSemicolon,
    ///
    #[token(":")]
    SymColon,
    ///
    #[token("`")]
    SymBacktick,
    ///
    #[token("?")]
    SymQuery,
    ///
    #[token("@")]
    SymAt,
    ///
    #[token("=")]
    SymEqual,
    ///
    #[token("==")]
    SymDoubleEqual,
    ///
    #[token("+=")]
    SymPlusEqual,
    ///
    #[token("-=")]
    SymDashEqual,
    ///
    #[token("*=")]
    SymStarEqual,
    ///
    #[token("/=")]
    SymSlashEqual,
    ///
    #[token("%=")]
    SymPercentEqual,
    ///
    #[token("&=")]
    SymAndEqual,
    ///
    #[token("|=")]
    SymPipeEqual,
    ///
    #[token("^=")]
    SymCaretEqual,
    ///
    #[token("!=")]
    SymBangEqual,
    ///
    #[token(".=")]
    SymCyclopsWalrus,
    ///
    #[token(":=")]
    SymWalrus,
    ///
    #[token("(")]
    SymOpenParen,
    ///
    #[token(")")]
    SymCloseParen,
    ///
    #[token("[")]
    SymOpenBracket,
    ///
    #[token("]")]
    SymCloseBracket,
    ///
    #[token("{")]
    SymOpenBrace,
    ///
    #[token("}")]
    SymCloseBrace,
    ///
    #[token("<")]
    SymOpenChev,
    ///
    #[token("<=")]
    SymOpenChevEqual,
    ///
    #[token("-<")]
    SymVacuum,
    ///
    #[token("<<")]
    SymDoubleOpenChev,
    ///
    #[token("<<=")]
    SymDoubleOpenChevEqual,
    ///
    #[token(">")]
    SymCloseChev,
    ///
    #[token(">=")]
    SymCloseChevEqual,
    ///
    #[token("->")]
    SymArrow,
    ///
    #[token(">>")]
    SymDoubleCloseChev,
    ///
    #[token(">>=")]
    SymDoubleCloseChevEqual,
    ///
    #[token(">->")]
    SymFletchedArrow,
    ///
    #[token("_")]
    SymUnderscore,
    ///
    #[token("\n")]
    #[token("\r\n")]
    #[token("\r")]
    SymNewline,
    ///
    #[regex(r"[`~!$%^&*\-=\+|;:,<.>/?]+", |lex| lex.slice().to_string())]
    SymCustom(String),

    ///
    #[token("true", |_lex| { true })]
    #[token("false", |_lex| { false })]
    WordBoolVal(bool),
    ///
    #[token("bool")]
    WordBoolType,
    ///
    #[token("byte")]
    WordByteType,
    ///
    #[token("nat")]
    WordNaturalType,
    ///
    #[token("int")]
    WordIntegerType,
    ///
    #[token("flo")]
    WordFloatingType,
    ///
    #[token("char")]
    WordCharacterType,
    ///
    #[token("str")]
    WordStringType,
    ///
    #[token("any")]
    WordAnyType,
    ///
    #[token("Self")]
    WordSelfType,
    ///
    #[token("self")]
    WordSelf,
    ///
    #[token("func")]
    WordFunction,
    ///
    #[token("type")]
    WordType,
    ///
    #[token("impl")]
    WordImplement,
    ///
    #[token("trait")]
    WordTrait,
    ///
    #[token("effect")]
    WordEffect,
    // ///
    // #[token("macro")]
    // WordMacro,
    ///
    #[token("pub")]
    WordPublic,
    ///
    #[token("prt")]
    WordProtected,
    ///
    #[token("if")]
    WordIf,
    ///
    #[token("elif")]
    WordElseIf,
    ///
    #[token("else")]
    WordElse,
    ///
    #[token("matches")]
    WordMatches,
    ///
    #[token("then")]
    WordThen,
    ///
    #[token("always")]
    WordAlways,
    ///
    #[token("while")]
    WordWhile,
    ///
    #[token("for")]
    WordFor,
    ///
    #[token("in")]
    WordIn,
    ///
    #[token("with")]
    WordWith,
    ///
    #[token("when")]
    WordWhen,
    ///
    #[token("do")]
    WordDo,
    ///
    #[token("is")]
    WordIs,
    ///
    #[token("use")]
    WordUse,
    ///
    #[token("as")]
    WordAs,
    ///
    #[token("end")]
    WordEnd,
    ///
    #[token("and")]
    WordAnd,
    ///
    #[token("or")]
    WordOr,
    ///
    #[regex(r"\p{XID_START}\p{XID_CONTINUE}", |lex| lex.slice().to_string())]
    WordIdentifier(String),

    ///
    #[regex(r"#[^(\n|\r\n|\r)]", |lex| lex.slice().strip_prefix('#').unwrap().to_string())]
    #[regex(r"#\[[^(\]#)]\]#", |lex| lex.slice().strip_prefix("#[").unwrap().strip_suffix("]#").unwrap().to_string())]
    Comment(String),
    ///
    #[regex(r"##[^(\n|\r\n|\r)]", |lex| lex.slice().strip_prefix("##").unwrap().to_string())]
    #[regex(r"##![^(\n|\r\n|\r)]", |lex| lex.slice().strip_prefix("##!").unwrap().to_string())]
    #[regex(r"##\[[^(\]#)]\]##", |lex| lex.slice().strip_prefix("##[").unwrap().strip_suffix("]##").unwrap().to_string())]
    #[regex(r"##!\[[^(\]#)]\]!##", |lex| lex.slice().strip_prefix("##![").unwrap().strip_suffix("]!##").unwrap().to_string())]
    DocComment(String),
}

#[derive(Debug, PartialEq)]
pub enum PartKind {
    Initial,
    Medial,
    Final,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {}

//--> Functions & Impls <--

impl<'a> Scanner<'a> {
    pub(crate) fn new(lexer: Lexer<'a, TokenKind>) -> Scanner<'a> {
        Scanner { inner: lexer.spanned() }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some((Ok(kind), span)) => Some(Ok(Token {
                kind,
                span,
                code: self.inner.slice().into(),
            })),
            Some((Err(mut e), span)) => {
                e.span = Some(span);
                Some(Err(e))
            },
            None => None,
        }
    }
}

fn process_char(lex: &mut Lexer<TokenKind>) -> Result<char, Error> {
    let chars: Vec<char> = lex.slice()
        .strip_prefix('\'').unwrap()
        .strip_suffix('\'').unwrap()
        .chars().collect();

    if chars.is_empty() {
        return Err(Error {
            span: None,
        })
    }

    if chars[0] == '\\' {
        if chars.len() < 2 {
            return Err(Error {
                span: None,
            })
        }

        match chars[1] {
            '\\' if chars.len() == 2 => Ok('\\'),
            '\'' if chars.len() == 2 => Ok('\''),
            '\"' if chars.len() == 2 => Ok('"'),
            '0' if chars.len() == 2 => Ok('\0'),
            't' if chars.len() == 2 => Ok('\t'),
            'n' if chars.len() == 2 => Ok('\n'),
            'r' if chars.len() == 2 => Ok('\r'),
            'x' => {
                // ASCII Escape Sequence

                if chars.len() < 4 {
                    return Err(Error {
                        span: None,
                    })
                } else if chars.len() > 4 {
                    return Err(Error {
                        span: None,
                    })
                }

                let mut value = String::new();
                value.push(chars[2]);
                value.push(chars[3]);

                match u8::from_str_radix(&value, 16) {
                    Ok(byte) if byte.is_ascii() => Ok(char::from_u32(byte as u32).unwrap()),
                    Ok(_) => Err(Error {
                        span: None,
                    }),
                    Err(e) => Err(Error {
                        span: None,
                    })
                }
            },
            'u' => {
                // UTF-8 Escape Sequence

                if chars.len() < 5 {
                    return Err(Error {
                        span: None,
                    })
                } else if chars.len() > 10 {
                    return Err(Error {
                        span: None,
                    })
                } else if chars[2] != '{' {
                    return Err(Error {
                        span: None,
                    })
                } else if chars[chars.len() - 1] != '}' {
                    return Err(Error {
                        span: None,
                    })
                }

                let mut value = String::new();
                for chr in &chars[3..chars.len() - 1] {
                    value.push(*chr);
                }

                match u32::from_str_radix(&value, 16) {
                    Ok(val) => match char::from_u32(val) {
                        Some(chr) => Ok(chr),
                        None => Err(Error {
                            span: None,
                        })
                    },
                    Err(e) => Err(Error {
                        span: None,
                    })
                }
            },
            chr if chars.len() == 2 => Ok(chr),
            _ => Err(Error {
                span: None,
            })
        }
    } else {
        if chars.len() != 1 {
            return Err(Error {
                span: None,
            })
        }

        Ok(chars[0])
    }
}

fn process_str(lex: &mut Lexer<TokenKind>) -> Result<String, Error> {
    let mut chars: Vec<char> = lex.slice()
        .strip_prefix('"').unwrap()
        .strip_suffix('"').unwrap()
        .chars().collect();

    if chars.is_empty() { return Ok("".into()) }
    
    let mut output = String::new();
    let mut errs = Vec::new();

    loop {
        // Getting to this point with an empty character stack is impossible, so unwrapping should be okay here.
        let chr0 = chars.pop().unwrap();

        if chr0 == '\\' {
            match chars.pop() {
                Some('\\') => output.push('\\'),
                Some('\'') => output.push('\''),
                Some('\"') => output.push('\"'),
                Some('0') => output.push('\0'),
                Some('t') => output.push('\t'),
                Some('n') => output.push('\n'),
                Some('r') => output.push('\r'),
                Some('x') => {
                    // ASCII Escape Sequence
                    let mut value = String::new();
                    match chars.pop() {
                        Some(chr) => value.push(chr),
                        None => errs.push(Error {
                            span: None,
                        }),
                    }
                    match chars.pop() {
                        Some(chr) => value.push(chr),
                        None => errs.push(Error {
                            span: None,
                        }),
                    }

                    match u8::from_str_radix(&value, 16) {
                        Ok(byte) if byte.is_ascii() => output.push(char::from_u32(byte as u32).unwrap()),
                        Ok(_) => errs.push(Error {
                            span: None,
                        }),
                        Err(e) => errs.push(Error {
                            span: None,
                        })
                    }
                },
                Some('u') => {
                    // UTF-8 Escape Sequence
                    if let Some('{') = chars.pop() {
                        let mut value = String::new();
                        loop {
                            match chars.pop() {
                                Some('}') => break,
                                Some(chr) => value.push(chr),
                                None => errs.push(Error {
                                    span: None,
                                }),
                            }
                        }

                        match u32::from_str_radix(&value, 16) {
                            Ok(val) => match char::from_u32(val) {
                                Some(chr) => output.push(chr),
                                None => errs.push(Error {
                                    span: None,
                                })
                            },
                            Err(e) => errs.push(Error {
                                span: None,
                            })
                        }
                    } else {
                        errs.push(Error {
                            span: None,
                        })
                    }
                },
                Some(chr) => output.push(chr),
                None => errs.push(Error {
                    span: None,
                }),
            }
        } else {
            output.push(chr0);
        }

        if chars.is_empty() { break }
    }

    if !errs.is_empty() {
        return Err(Error {
            span: None,
        })
    }

    Ok(output)
}

fn process_str_interp(lex: &mut Lexer<TokenKind>) -> Result<StrInterpPart, Error> {
    let slice = lex.slice();
    let (mut chars, part): (Vec<char>, PartKind) = if slice.starts_with('"') {
        (slice.strip_prefix('"').unwrap()
            .strip_suffix("\\{").unwrap()
            .chars().collect(), PartKind::Initial)
    } else if slice.ends_with("\\{") {
        (slice.strip_prefix('}').unwrap()
            .strip_suffix("\\{").unwrap()
            .chars().collect(), PartKind::Medial)
    } else if slice.ends_with('"') {
        (slice.strip_prefix('}').unwrap()
            .strip_suffix('"').unwrap()
            .chars().collect(), PartKind::Final)
    } else { unreachable!() };

    if chars.is_empty() { return Ok(StrInterpPart { string: "".into(), kind: part }) }

    let mut output = String::new();
    let mut errs = Vec::new();

    loop {
        // Getting to this point with an empty character stack is impossible, so unwrapping should be okay here.
        let chr0 = chars.pop().unwrap();

        if chr0 == '\\' {
            match chars.pop() {
                Some('\\') => output.push('\\'),
                Some('\'') => output.push('\''),
                Some('\"') => output.push('\"'),
                Some('0') => output.push('\0'),
                Some('t') => output.push('\t'),
                Some('n') => output.push('\n'),
                Some('r') => output.push('\r'),
                Some('x') => {
                    // ASCII Escape Sequence
                    let mut value = String::new();
                    value.push(chars.pop().ok_or_else(|| Error {
                        span: None,
                    })?);
                    value.push(chars.pop().ok_or_else(|| Error {
                        span: None,
                    })?);

                    match u8::from_str_radix(&value, 16) {
                        Ok(byte) if byte.is_ascii() => output.push(char::from_u32(byte as u32).unwrap()),
                        Ok(_) => errs.push(Error {
                            span: None,
                        }),
                        Err(e) => errs.push(Error {
                            span: None,
                        })
                    }
                },
                Some('u') => {
                    // UTF-8 Escape Sequence
                    if let Some('{') = chars.pop() {
                        let mut value = String::new();
                        loop {
                            match chars.pop() {
                                Some('}') => break,
                                Some(chr) => value.push(chr),
                                None => errs.push(Error {
                                    span: None,
                                }),
                            }
                        }

                        match u32::from_str_radix(&value, 16) {
                            Ok(val) => match char::from_u32(val) {
                                Some(chr) => output.push(chr),
                                None => errs.push(Error {
                                    span: None,
                                })
                            },
                            Err(e) => errs.push(Error {
                                span: None,
                            })
                        }
                    } else {
                        errs.push(Error {
                            span: None,
                        })
                    }
                },
                Some(chr) => output.push(chr),
                None => errs.push(Error {
                    span: None,
                }),
            }
        } else {
            output.push(chr0);
        }

        if chars.is_empty() { break }
    }

    if !errs.is_empty() {
        return Err(Error {
            span: None,
        })
    }

    Ok(StrInterpPart { string: output, kind: part })
}

fn char_to_byte(chr: char) -> Option<u8> {
    let mut buf = Vec::new();
    chr.encode_utf8(&mut buf);
    if buf.len() == 1 { Some(buf[0]) } else { None }
}

fn process_byte(lex: &mut Lexer<TokenKind>) -> Result<u8, Error> {
    let chars: Vec<char> = lex.slice()
        .strip_prefix('\'').unwrap()
        .strip_suffix('\'').unwrap()
        .chars().collect();

    if chars.is_empty() {
        return Err(Error {
            span: None,
        })
    }

    if chars[0] == '\\' {
        if chars.len() < 2 {
            return Err(Error {
                span: None,
            })
        }

        match chars[1] {
            '\\' if chars.len() == 2 => Ok(0x5c),
            '\'' if chars.len() == 2 => Ok(0x27),
            '\"' if chars.len() == 2 => Ok(0x22),
            '0' if chars.len() == 2 => Ok(0x00),
            't' if chars.len() == 2 => Ok(0x09),
            'n' if chars.len() == 2 => Ok(0x0a),
            'r' if chars.len() == 2 => Ok(0x0d),
            'x' => {
                // Byte Escape Sequence

                if chars.len() < 4 {
                    return Err(Error {
                        span: None,
                    })
                } else if chars.len() > 4 {
                    return Err(Error {
                        span: None,
                    })
                }

                let mut value = String::new();
                value.push(chars[2]);
                value.push(chars[3]);

                match u8::from_str_radix(&value, 16) {
                    Ok(byte) => Ok(byte),
                    Err(e) => Err(Error {
                        span: None,
                    })
                }
            },
            chr if chars.len() == 2 => char_to_byte(chr).ok_or_else(|| Error {
                span: None,
            }),
            _ => Err(Error {
                span: None,
            })
        }
    } else {
        if chars.len() != 1 {
            return Err(Error {
                span: None,
            })
        }

        char_to_byte(chars[0]).ok_or_else(|| Error {
            span: None,
        })
    }
}

fn process_byte_str(lex: &mut Lexer<TokenKind>) -> Result<Vec<u8>, Error> {
    let mut chars: Vec<char> = lex.slice()
        .strip_prefix('"').unwrap()
        .strip_suffix('"').unwrap()
        .chars().collect();

    if chars.is_empty() { return Ok(vec![]) }
    
    let mut output = Vec::new();
    let mut errs = Vec::new();

    loop {
        // Getting to this point with an empty character stack is impossible, so unwrapping should be okay here.
        let chr0 = chars.pop().unwrap();

        if chr0 == '\\' {
            match chars.pop() {
                Some('\\') => output.push(0x5c),
                Some('\'') => output.push(0x27),
                Some('\"') => output.push(0x22),
                Some('0') => output.push(0x00),
                Some('t') => output.push(0x09),
                Some('n') => output.push(0x0a),
                Some('r') => output.push(0x0d),
                Some('x') => {
                    // Byte Escape Sequence
                    let mut value = String::new();
                    match chars.pop() {
                        Some(chr) => value.push(chr),
                        None => errs.push(Error {
                            span: None,
                        }),
                    }
                    match chars.pop() {
                        Some(chr) => value.push(chr),
                        None => errs.push(Error {
                            span: None,
                        }),
                    }

                    match u8::from_str_radix(&value, 16) {
                        Ok(byte) => output.push(byte),
                        Err(e) => errs.push(Error {
                            span: None,
                        })
                    }
                },
                Some(chr) => match char_to_byte(chr) {
                    Some(byte) => output.push(byte),
                    None => errs.push(Error {
                        span: None,
                    })
                },
                None => errs.push(Error {
                    span: None,
                }),
            }
        } else {
            match char_to_byte(chr0) {
                Some(byte) => output.push(byte),
                None => errs.push(Error {
                    span: None,
                })
            }
        }

        if chars.is_empty() { break }
    }

    if !errs.is_empty() {
        return Err(Error {
            span: None,
        })
    }

    Ok(output)
}

fn process_raw_byte_str(lex: &mut Lexer<TokenKind>) -> Result<Vec<u8>, Error> {
    let mut iter = lex.slice()
        .strip_prefix("br#\"").unwrap()
        .strip_suffix("\"#rb").unwrap()
        .chars()
        .map(|ch| {
            char_to_byte(ch)
                .ok_or_else(|| Error {
                    span: None,
                })
        });
    
    let mut bytes = Vec::new();
    let mut errs = Vec::new();

    for item in iter {
        match item {
            Ok(byte) => bytes.push(byte),
            Err(e) => errs.push(e),
        }
    }

    if !errs.is_empty() {
        return Err(Error {
            span: None,
        })
    }

    Ok(bytes)
}