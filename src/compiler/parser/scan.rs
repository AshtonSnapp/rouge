//!

//--> Imports & Modules <--

use std::{
    cmp::Ordering, num::ParseFloatError, str::FromStr
};

use logos::{
    Lexer,
    Logos,
    Span,
    SpannedIter,
};

//--> Structs <--

pub(crate) struct Scanner<'source> {
    inner: SpannedIter<'source, TokenKind>
}

///
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Token {
    kind: TokenKind,
    span: Span,
    code: String,
}

///
#[derive(Clone, Default, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    span: Option<Span>,
    code: Option<String>,
    help: Option<String>,
}

//--> Enums <--

///
#[derive(Clone, Logos, PartialEq, PartialOrd)]
#[logos(error = Error)]
#[logos(skip r"[ \t]")]
pub enum TokenKind {
    #[regex(r"'([^']|\\')'", callback = |l| {
        
        let chars: Vec<char> = l.slice()
            .strip_prefix('\'').unwrap()
            .strip_suffix('\'').unwrap()
            .chars().collect();

        if chars.is_empty() {
            return Err(Error {
                kind: ErrorKind::CharEmpty,
                span: None,
                code: Some(l.slice().into()),
                help: None,
            });
        }

        if chars.len() != 1 && chars[0] != '\\' {
            return Err(Error {
                kind: ErrorKind::CharOverflow,
                span: None,
                code: Some(l.slice().into()),
                help: Some("Character literals, which are surrounded by single quotes, cannot contain multiple characters outside of a single escape sequence.".into())
            });
        }

        if chars.len() == 1 && chars[0] == '\\' {
            return Err(Error {
                kind: ErrorKind::IncompleteEscape,
                span: None,
                code: Some(l.slice().into()),
                help: Some("'\\' indicates an escape sequence, and must be followed by at least one more character.".into())
            });
        }

        if chars.len() == 1 && chars[0] != '\\' {
            return Ok(chars[0]);
        }

        match chars[1] {
            '\\' if chars.len() == 2 => Ok('\\'),
            '\'' if chars.len() == 2 => Ok('\''),
            '0' if chars.len() == 2 => Ok('\0'),
            't' if chars.len() == 2 => Ok('\t'),
            'n' if chars.len() == 2 => Ok('\n'),
            'r' if chars.len() == 2 => Ok('\r'),
            'x' if chars.len() == 4 => { // ASCII escape sequence
                
                let mut val = String::new();
                val.push(chars[2]);
                val.push(chars[3]);

                let num = u8::from_str_radix(&val, 16).map_err(|e| Error {
                    kind: ErrorKind::InvalidASCIIOrByteEscape(Some(e)),
                    span: None,
                    code: Some(l.slice().into()),
                    help: Some("Make sure you are using valid hexadecimal.".into())
                })?;

                if !num.is_ascii() {
                    return Err(Error {
                        kind: ErrorKind::InvalidASCIIOrByteEscape(None),
                        span: None,
                        code: Some(l.slice().into()),
                        help: Some("Valid ASCII characters are within the range from 0 to 127, or hex 00 to 7F. For values beyond this point, use a Unicode escape.".into())
                    })
                }

                char::from_u32(num as u32).ok_or_else(|| Error {
                    kind: ErrorKind::InvalidASCIIOrByteEscape(None),
                    span: None,
                    code: Some(l.slice().into()),
                    help: None
                })

            },
            'x' if chars.len() < 4 => Err(Error {
                kind: ErrorKind::IncompleteEscape,
                span: None,
                code: Some(l.slice.into()),
                help: Some("ASCII escape sequences require four characters total, in the format of '\\xHH' where H is a hexadecimal digit.".into())
            }),
            'u' if (5..=10).contains(chars.len()) => { // Unicode escape sequence
                
                if chars[2] != '{' {
                    return Err(Error {
                        kind: ErrorKind::InvalidUnicodeEscape(None),
                        span: None,
                        code: Some(l.slice().into()),
                        help: Some("Unicode escape sequences follow the format of '\\u{H+}'. You are missing the '{'.".into())
                    });
                } else if chars[chars.len() - 1] != '}' {
                    return Err(Error {
                        kind: ErrorKind::InvalidUnicodeEscape(None),
                        span: None,
                        code: Some(l.slice().into()),
                        help: Some("Unicode escape sequences follow the format of '\\u{H+}'. You are missing the '}'.".into())
                    });
                }

                let mut val = String::new();
                for chr in chars[3..chars.len() - 2] {
                    val.push(chr);
                }

                char::from_u32(u32::from_str_radix(&val, 16).map_err(|e| Error {
                    kind: ErrorKind::InvalidUnicodeEscape(Some(e)),
                    span: None,
                    code: Some(l.slice().into()),
                    help: Some("Make sure the desired Unicode code point is represented as a hexadecimal number.".into())
                })?).ok_or_else(|| Error {
                    kind: ErrorKind::InvalidUnicodeEscape(None),
                    span: None,
                    code: Some(l.slice().into()),
                    help: Some("Make sure your desired Unicode code point has a valid UTF-8 representation.".into())
                })

            },
            'u' if chars.len() < 5 => Err(Error {
                kind: ErrorKind::IncompleteEscape,
                span: None,
                code: Some(l.slice().into()),
                help: Some("Unicode escape sequences require five to ten characters total, in the format of '\\u{H+}' where H+ is one to six hexadecimal digits.".into())
            })
            chr if chars.len() == 2 => Ok(chr),
            _ => Err(Error {
                kind: ErrorKind::CharOverflow,
                span: None,
                code: Some(l.slice().into()),
                help: Some("Only one character or escape sequence can be contained in a character literal.".into()),
            })
        }

    })]
    LitChar(char),
    #[regex(r#""([^"]|\\")*""#, callback = |l| {
        
        let mut chars: Vec<char> = l.slice()
            .strip_prefix('"').unwrap()
            .strip_suffix('"').unwrap()
            .chars().rev().collect();

        if chars.is_empty() { return Ok("".to_string()); }

        let mut string = String::new();
        let mut errors = Vec::new();

        'char: loop {
            let char0 = chars.pop().unwrap();

            if char0 != '\\' {
                string.push(char0);
            } else {
                match chars.pop() {
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some('0') => string.push('\0'),
                    Some('t') => string.push('\t'),
                    Some('n') => string.push('\n'),
                    Some('r') => string.push('\r'),
                    Some('x') => { // ASCII escape sequence
                        let mut val = String::new();
                        match chars.pop() {
                            Some(chr) => val.push(chr),
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::IncompleteEscape,
                                    span: None,
                                    code: Some(l.slice().into()),
                                    help: Some("ASCII escape sequences consist of four characters: a backslash, the character 'x', and two hexadecimal digits.".into())
                                });
                                break 'char;
                            }
                        }
                        match chars.pop() {
                            Some(chr) => val.push(chr),
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::IncompleteEscape,
                                    span: None,
                                    code: Some(l.slice().into()),
                                    help: Some("ASCII escape sequences consist of four characters: a backslash, the character 'x', and two hexadecimal digits.".into())
                                });
                                break 'char;
                            }
                        }
                        match u8::from_str_radix(&val, 16) {
                            Ok(num) if num.is_ascii() => match char::from_u32(num as u32) {
                                Some(chr) => string.push(chr),
                                None => errors.push(Error {
                                    kind: ErrorKind::InvalidASCIIOrByteEscape(None),
                                    span: None,
                                    code: Some(l.slice().into()),
                                    help: None,
                                })
                            },
                            Ok(_) => errors.push(Error {
                                kind: ErrorKind::InvalidASCIIOrByteEscape(None),
                                span: None,
                                code: Some(l.slice().into()),
                                help: Some("Valid ASCII characters are within the range from 0 to 127, or hex 00 to 7F. For values beyond this point, use a Unicode escape.".into())
                            })
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidASCIIOrByteEscape(Some(e)),
                                span: None,
                                code: Some(l.slice().into()),
                                help: None,
                            })
                        }
                    },
                    Some('u') => { // Unicode escape sequence
                        match chars.pop() {
                            Some('{') => {
                                
                                let mut val = String::new();
                                
                                loop {
                                    match chars.pop() {
                                        Some('}') => break,
                                        Some(other) => val.push(other),
                                        None => {
                                            errors.push(Error {
                                                kind: ErrorKind::IncompleteEscape,
                                                span: None,
                                                code: Some(l.slice().into()),
                                                help: Some("Unicode escape sequences consist of 5 to 10 characters in the format '\\u{H+}' where H+ is 1 to 6 hexadecimal digits.".into()),
                                            })
                                        }
                                    }
                                }
                                
                                match u32::from_str_radix(&val, 16) {
                                    Ok(num) => match char::from_u32(num) {
                                        Some(chr) => string.push(chr),
                                        None => errors.push(Error {
                                            kind: ErrorKind::InvalidUnicodeEscape(None),
                                            span: None,
                                            code: Some(l.slice().into()),
                                            help: None,
                                        })
                                    },
                                    Err(e) => errors.push(Error {
                                        kind: ErrorKind::InvalidUnicodeEscape(Some(e)),
                                        span: None,
                                        code: Some(l.slice().into()),
                                        help: None
                                    })
                                }

                            },
                            Some(other) => errors.push(Error {
                                kind: ErrorKind::InvalidUnicodeEscape(None),
                                span: None,
                                code: Some(l.slice().into()),
                                help: Some(format!("Expected character '{{', found character '{other}' instead."))
                            })
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::IncompleteEscape,
                                    span: None,
                                    code: Some(l.slice().into()),
                                    help: Some("Unicode escape sequences consist of 5 to 10 characters in the format '\\u{H+}' where H+ is 1 to 6 hexadecimal digits.".into()),
                                })
                                break 'char;
                            }
                        }
                    },
                    Some(chr) => string.push(chr),
                    None => errors.push(Error {
                        kind: ErrorKind::IncompleteEscape,
                        span: None,
                        code: Some(l.slice().into()),
                        help: Some("'\\' indicates an escape sequence, and must be followed by at least one more character.".into())
                    })
                }
            }

            if chars.is_empty() { break; }
        }

        if !errors.is_empty() {

            if errors.len() == 1 {
                return Err(errors.get(0).unwrap().clone());
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                span: None,
                code: Some(l.slice().into()),
                help: None,
            });

        }

        Ok(string)

    })]
    #[regex(r##"r#"[^("#)]"#"##, callback = |l| {
        
        l.slice()
            .strip_prefix("r#\"").unwrap()
            .strip_suffix("\"#").unwrap()
            .to_string()

    })]
    LitStr(String),
    #[regex(r"b'([^']|\\')'", callback = |l| {
        let chars: Vec<char> = l.slice()
            .strip_prefix("b'").unwrap()
            .strip_suffix('\'').unwrap()
            .chars().collect();

        if chars.is_empty() {
            return Err(Error {
                kind: ErrorKind::CharEmpty,
                span: None,
                code: Some(l.slice().into()),
                help: None,
            });
        }

        if chars.len() != 1 && chars[0] != '\\' {
            return Err(Error {
                kind: ErrorKind::CharOverflow,
                span: None,
                code: Some(l.slice().into()),
                help: Some("Byte literals, which are surrounded by single quotes with a prefixed 'b', cannot contain multiple characters outside of a single escape sequence.".into())
            });
        }

        if chars.len() == 1 && chars[0] == '\\' {
            return Err(Error {
                kind: ErrorKind::IncompleteEscape,
                span: None,
                code: Some(l.slice().into()),
                help: Some("'\\' indicates an escape sequence, and must be followed by at least one more character.".into())
            });
        }

        if chars.len() == 1 && chars[0] != '\\' {
            return chr_to_byte(chars[0]).map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            });
        }

        match chars[1] {
            '\\' if chars.len() == 2 => chr_to_byte('\\').map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            '\'' if chars.len() == 2 => chr_to_byte('\'').map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            '0' if chars.len() == 2 => chr_to_byte('\0').map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            't' if chars.len() == 2 => chr_to_byte('\t').map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            'n' if chars.len() == 2 => chr_to_byte('\n').map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            'r' if chars.len() == 2 => chr_to_byte('\r').map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            'x' if chars.len() == 4 => { // Byte escape sequence
                
                let mut val = String::new();
                val.push(chars[2]);
                val.push(chars[3]);

                u8::from_str_radix(&val, 16).map_err(|e| Error {
                    kind: ErrorKind::InvalidASCIIOrByteEscape(Some(e)),
                    span: None,
                    code: Some(l.slice().into()),
                    help: Some("Make sure you are using valid hexadecimal.".into())
                })

            },
            'x' if chars.len() < 4 => Err(Error {
                kind: ErrorKind::IncompleteEscape,
                span: None,
                code: Some(l.slice.into()),
                help: Some("ASCII escape sequences require four characters total, in the format of '\\xHH' where H is a hexadecimal digit.".into())
            }),
            chr if chars.len() == 2 => chr_to_byte(chr).map_err(|e| Error {
                code: Some(l.slice().into()),
                ..e
            }),
            _ => Err(Error {
                kind: ErrorKind::CharOverflow,
                span: None,
                code: Some(l.slice().into()),
                help: Some("Only one character or escape sequence can be contained in a character literal.".into()),
            })
        }
    })]
    LitByte(u8),
    #[regex(r#"b"([^"]|\\")*""#, callback = |l| {
                
        let mut chars: Vec<char> = l.slice()
            .strip_prefix('"').unwrap()
            .strip_suffix('"').unwrap()
            .chars().rev().collect();

        if chars.is_empty() { return Ok("".to_string()); }

        let mut string = Vec::new();
        let mut errors = Vec::new();

        'char: loop {
            let char0 = chars.pop().unwrap();

            if char0 != '\\' {
                string.push(char0);
            } else {
                match chars.pop() {
                    Some('\\') => match chr_to_byte('\\') {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(Error {
                            code: Some(l.slice().into()),
                            ..e
                        }),
                    },
                    Some('"') => match chr_to_byte('"') {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(Error {
                            code: Some(l.slice().into()),
                            ..e
                        }),
                    },
                    Some('0') => match chr_to_byte('\0') {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(Error {
                            code: Some(l.slice().into()),
                            ..e
                        }),
                    },
                    Some('t') => match chr_to_byte('\t') {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(Error {
                            code: Some(l.slice().into()),
                            ..e
                        }),
                    },
                    Some('n') => match chr_to_byte('\n') {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(Error {
                            code: Some(l.slice().into()),
                            ..e
                        }),
                    },
                    Some('r') => match chr_to_byte('\r') {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(Error {
                            code: Some(l.slice().into()),
                            ..e
                        }),
                    },
                    Some('x') => { // ASCII escape sequence
                        let mut val = String::new();
                        match chars.pop() {
                            Some(chr) => val.push(chr),
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::IncompleteEscape,
                                    span: None,
                                    code: Some(l.slice().into()),
                                    help: Some("Byte escape sequences consist of four characters: a backslash, the character 'x', and two hexadecimal digits.".into())
                                });
                                break 'char;
                            }
                        }
                        match chars.pop() {
                            Some(chr) => val.push(chr),
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::IncompleteEscape,
                                    span: None,
                                    code: Some(l.slice().into()),
                                    help: Some("Byte escape sequences consist of four characters: a backslash, the character 'x', and two hexadecimal digits.".into())
                                });
                                break 'char;
                            }
                        }
                        match u8::from_str_radix(&val, 16) {
                            Ok(byte) => string.push(byte),
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidASCIIOrByteEscape(Some(e)),
                                span: None,
                                code: Some(l.slice().into()),
                                help: None,
                            })
                        }
                    },
                    Some(chr) => match chr_to_byte(chr) {
                        Ok(byte) => string.push(byte),
                        Err(e) => errors.push(e),
                    },
                    None => errors.push(Error {
                        kind: ErrorKind::IncompleteEscape,
                        span: None,
                        code: Some(l.slice().into()),
                        help: Some("'\\' indicates an escape sequence, and must be followed by at least one more character.".into())
                    })
                }
            }

            if chars.is_empty() { break; }
        }

        if !errors.is_empty() {

            if errors.len() == 1 {}

            Err(Error {
                kind: ErrorKind::Multiple(errors),
                span: None,
                code: Some(l.slice().into()),
                help: None,
            })

        }

        Ok(string)

    })]
    #[regex(r##"br#"[^("#)]"#"##, callback = |l| {
        
        l.slice()
            .strip_prefix("br#\"").unwrap()
            .strip_suffix("\"#").unwrap()
            .chars()
            .map(|c| chr_to_byte(c))
            .try_collect()

    })]
    LitByteStr(Vec<u8>),
    #[regex(r"0[bB][01][01_]*", callback = |l| {
        u128::from_str_radix(l.slice().to_lowercase().strip_prefix("0b"), 2).map_err(|e| Error {
            kind: ErrorKind::InvalidNumber(e),
            span: None,
            code: Some(l.slice().into()),
            help: None,
        })
    })]
    #[regex(r"0[oO][0-7][0-7_]*", callback = |l| {
        u128::from_str_radix(l.slice().to_lowercase().strip_prefix("0o"), 8).map_err(|e| Error {
            kind: ErrorKind::InvalidNumber(e),
            span: None,
            code: Some(l.slice().into()),
            help: None,
        })
    })]
    #[regex(r"[0-9][0-9_]*", callback = |l| {
        u128::from_str(l.slice()).map_err(|e| Error {
            kind: ErrorKind::InvalidNumber(e),
            span: None,
            code: Some(l.slice().into()),
            help: None,
        })
    })]
    #[regex(r"0[xX][0-9a-fA-F][0-9a-fA-F_]*", callback = |l| {
        u128::from_str_radix(l.slice().to_lowercase().strip_prefix("0x"), 16).map_err(|e| Error {
            kind: ErrorKind::InvalidNumber(e),
            span: None,
            code: Some(l.slice().into()),
            help: None,
        })
    })]
    LitNum(u128),
    #[regex(r"[0-9][0-9_]*((\.[0-9][0-9_]*)|([eE][\+\-][0-9][0-9_]*)|(\.[0-9][0-9_]*[eE][\+\-][0-9][0-9_]*))", callback = |l| {
        f64::from_str(l.slice()).map_err(|e| Error {
            kind: ErrorKind::InvalidFloat(e),
            span: None,
            code: Some(l.slice().into()),
            help: None
        })
    })]
    LitFlo(f64),

    /// Used to indicate block labels
    #[token("`")]
    SymTick,
    /// Used for bitwise and logical not, as well as comptime macros (TODO)
    #[token("!")]
    SymBang,
    /// Used for attribute macros (TODO)
    #[token("@")]
    SymAttribute,
    /// Used for modulo/remainder
    #[token("%")]
    SymPercent,
    /// Used for divisibility check
    #[token("%%")]
    SymDoublePercent,
    /// Used for bitwise exclusive or
    #[token("^")]
    SymCaret,
    /// Used for bitwise and
    #[token("&")]
    SymAnd,
    /// Used for logical and
    #[token("&&")]
    SymDoubleAnd,
    /// Used for multiplication
    #[token("*")]
    SymStar,
    /// Used for function and type arguments, and struct literals
    #[token("(")]
    SymOpenParen,
    /// Used for function and type arguments, and struct literals
    #[token(")")]
    SymCloseParen,
    /// Used for negation and subtraction
    #[token("-")]
    SymDash,
    /// Indicates a discarded return value
    #[token("_", priority = 20)]
    SymUnderscore,
    /// Used to mutate values
    #[token("=")]
    SymEqual,
    /// Used to check for equality
    #[token("==")]
    SymDoubleEqual,
    /// Used to check for inequality
    #[token("!=")]
    SymBangEqual,
    /// Used for modulo/remainder assignment
    #[token("%=")]
    SymPercentEqual,
    /// Used for bitwise exclusive or assignment
    #[token("^=")]
    SymCaretEqual,
    /// Used for bitwise and assignment
    #[token("&=")]
    SymAndEqual,
    /// Used for multiply assignment
    #[token("*=")]
    SymStarEqual,
    /// Used for subtract assignment
    #[token("-=")]
    SymDashEqual,
    /// Used for addition
    #[token("+")]
    SymPlus,
    /// Used for addition assignment
    #[token("+=")]
    SymPlusEqual,
    /// Used for collections
    #[token("[")]
    SymOpenBracket,
    /// Used for collections
    #[token("]")]
    SymCloseBracket,
    /// Optionally used for code
    #[token("{")]
    SymOpenBrace,
    /// Optionally used for code
    #[token("}")]
    SymCloseBrace,
    /// Used for bitwise or
    #[token("|")]
    SymPipe,
    /// Used for logical or
    #[token("||")]
    SymDoublePipe,
    /// Used for bitwise or assignment
    #[token("|=")]
    SymPipeEqual,
    /// Optionally used to indicate the end of a statement
    #[token(";")]
    SymSemicolon,
    /// Used for type labels
    #[token(":")]
    SymColon,
    /// Used for namespace access
    #[token("::")]
    SymDoubleColon,
    /// Used for type-inferred variable declaration
    #[token(":=")]
    SymWalrus,
    /// Optionally used to separate collection elements
    #[token(",")]
    SymComma,
    /// Used for member access
    #[token(".")]
    SymDot,
    /// Used for method call assignment (i.e. `num .= pow(2)`, which calls `num.pow(2)` and assigns the output to `num`)
    #[token(".=")]
    SymCyclopsWalrus,
    /// Used for less than comparison
    #[token("<")]
    SymLessThan,
    /// Used for bitwise shift left
    #[token("<<")]
    SynShiftLeft,
    /// Used for bitwise shift left assignment
    #[token("<<=")]
    SymShiftLeftEqual,
    /// Used for less than or equal comparison
    #[token("<=")]
    SymLessThanEqual,
    /// Used for function effect types
    #[token("-<")]
    SymVacuum,
    /// Used for greater than comparison
    #[token(">")]
    SymGreaterThan,
    /// Used for bitwise shift right
    #[token(">>")]
    SymShiftRight,
    /// Used for bitwise shift right assignment
    #[token(">>=")]
    SymShiftRightEqual,
    /// Used for greater than or equal comparison
    #[token(">=")]
    SymGreaterThanEqual,
    /// Used for function return types
    #[token("->")]
    SymArrow,
    /// Used for monadic bind
    #[token(">->")]
    SymFletchedArrow,
    /// Used for division
    #[token("/")]
    SymSlash,
    /// Used for divide assignment
    #[token("/=")]
    SymSlashEqual,
    /// Used for returning if a monadic type in a non-happy-path state is encountered, but requires the function returns the same monadic type
    #[token("?")]
    SymTry,
    /// Used for throwing if a monadic type in a non-happy-path state is encountered
    #[token("?!")]
    SymUnwrap,
    /// Optionally separates statements and elements
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    SymNewline,

    /// A true or false value
    #[token("true", callback = || true)]
    #[token("false", callback = || false)]
    WordBooleanValue(bool),
    /// The type that respresents a true or false value
    #[token("bool")]
    WordBooleanType,
    /// The type that represents an arbitrarily sized unsigned integer
    #[token("nat")]
    WordNatural,
    /// The type that represents an 8-bit unsigned integer
    #[token("nat8")]
    Word8BitNatural,
    /// The type that represents a 16-bit unsigned integer
    #[token("nat16")]
    Word16BitNatural,
    /// The type that represents a 32-bit unsigned integer
    #[token("nat32")]
    Word32BitNatural,
    /// The type that represents a 64-bit unsigned integer
    #[token("nat64")]
    Word64BitNatural,
    /// The type that represents a 128-bit unsigned integer
    #[token("nat128")]
    Word128BitNatural,
    /// The type that represents an arbitrarily sized signed integer
    #[token("int")]
    WordInteger,
    /// The type that represents an 8-bit signed integer
    #[token("int8")]
    Word8BitInteger,
    /// The type that represents a 16-bit signed integer
    #[token("int16")]
    Word16BitInteger,
    /// The type that represents a 32-bit signed integer
    #[token("int32")]
    Word32BitInteger,
    /// The type that represents a 64-bit signed integer
    #[token("int64")]
    Word64BitInteger,
    /// The type that represents a 128-bit signed integer
    #[token("int128")]
    Word128BitInteger,
    /// The type that represents an arbitrarily sized floating point number
    #[token("flo")]
    WordFloat,
    /// The type that represents a 16-bit (half-precision) floating point number
    #[token("flo16")]
    Word16BitFloat,
    /// The type that represents a 32-bit (single-precision) floating point number
    #[token("flo32")]
    Word32BitFloat,
    /// The type that represents a 64-bit (double-precision) floating point number
    #[token("flo64")]
    Word64BitFloat,
    /// The type that respresents a Unicode code point in UTF-8
    #[token("char")]
    WordCharacter,
    /// The type that represents a string of Unicode code points in UTF-8
    #[token("str")]
    WordString,
    /// The type that is implementing something.
    #[token("Self")]
    WordSelfType,
    /// The instance of the type a method belongs to.
    #[token("self")]
    WordSelf,
    /// Marks a mutable variable.
    #[token("mut")]
    WordMut,
    /// Marks a constant, which is an immutable value that must be known at compile time.
    #[token("const")]
    WordConst,
    /// Marks a type definition.
    #[token("type")]
    WordType,
    /// Marks a function definition.
    #[token("func")]
    WordFunc,
    /// Marks a trait definition.
    #[token("trait")]
    WordTrait,
    /// Marks an implementation block, used for adding associated functions, methods, associated types, or constants to a type, 
    #[token("impl")]
    WordImpl,
    /// Marks an effect definition.
    #[token("effect")]
    WordEffect,
    #[token("mod")]
    WordModule,
    //#[token("macro")]
    //WordMacro,
    /// Marks an item (a constant, type, function, trait, or effect) as public (directly accessible by code outside of the package)
    #[token("pub")]
    WordPublic,
    /// Marks an item (a constant, type, function, trait, or effect) as protected (directly accessible only by code inside the package, or inside of any stricter scope)
    #[token("prt")]
    WordProtected,
    /// Indicates the first branch of a conditional block.
    #[token("if")]
    WordIf,
    /// Indicates the nth branch of a conditional block.
    #[token("elif")]
    WordElseIf,
    /// Indicates the final branch of a conditional block.
    #[token("else")]
    WordElse,
    /// Separates the condition and body of a conditional branch.
    #[token("then")]
    WordThen,
    /// Indicates pattern matching.
    #[token("matches")]
    WordMatches,
    /// Indicates an infinite loop.
    #[token("always")]
    WordAlways,
    /// Indicates a conditional loop.
    #[token("while")]
    WordWhile,
    /// Indicates an iterator loop.
    #[token("for")]
    WordFor,
    /// Indicates the collection of an iterator loop, or a postfix block of an effect handler.
    #[token("in")]
    WordIn,
    /// Indicates a scope with restricted access to outside variables, or a prefix block of an effect handler.
    #[token("with")]
    WordWith,
    /// Indicates an effect handler.
    #[token("when")]
    WortWhen,
    /// Separates the signature and body of a type, trait, or effect definition.
    #[token("is")]
    WordIs,
    /// Separates the signature and body of a function or loop.
    #[token("do")]
    WordDo,
    /// Ends a block of code.
    #[token("end")]
    WordEnd,
    /// Indicates packages, modules, and items used by the current module.
    #[token("use")]
    WordUse,
    /// Aliases used packages, modules, and items, or variables being passed into a with block.
    #[token("as")]
    WordAs,
    /// Boolean and.
    #[token("and")]
    WordAnd,
    /// Boolean or.
    #[token("or")]
    WordOr,
    /// A user-specified identifier.
    #[regex(r"[\p{XID_Start}_]\p{XID_Continue}*", callback = |l| l.slice().to_string())]
    WordIdentifier(String),

    /// A comment. This is discarded immediately upon encountering it.
    #[regex(r"#[^\n]", callback = |l| l.slice().strip_prefix('#').unwrap().to_string())]
    #[regex(r"#\[[^(\]#)]\]#", callback = |l| l.slice().strip_prefix("#[").unwrap().strip_suffix("]#").unwrap().to_string())]
    Comment(String),
    /// A documentation comment. This is kept until after macro expansion (TODO), after which it is discarded.
    /// 
    /// `## ...` and `##[ ... ]##` are regular documentation comments. They document the item below them.
    /// `##! ...` and `##![ ... ]!##` are top-level documentation comments. They document the item they are within, and are generally
    /// only used for module documentation.
    #[regex(r"##[^\n]", callback = |l| l.slice().strip_prefix("##").unwrap().to_string())]
    #[regex(r"##\[[^(\]#)]\]##", callback = |l| l.slice().strip_prefix("##[").unwrap().strip_suffix("]##").unwrap().to_string())]
    #[regex(r"##![^\n]", callback = |l| l.slice().strip_prefix("##!").unwrap().to_string())]
    #[regex(r"##!\[[^(\]#)]\]!##", callback = |l| l.slice().strip_prefix("##![").unwrap().strip_suffix("]!##").unwrap().to_string())]
    DocumentationComment(String),

}

///
#[derive(Clone, Default, PartialEq)]
pub enum ErrorKind {
    CharEmpty,
    CharOverflow,
    IncompleteEscape,
    InvalidASCIIOrByteEscape(Option<ParseIntError>),
    InvalidUnicodeEscape(Option<ParseIntError>),
    InvalidForByteLiteral,
    InvalidNumber(ParseIntError),
    InvalidFloat(ParseFloatError),
    Multiple(Vec<Error>),
}

//--> Functions & Impls <--

impl<'source> Scanner<'source> {
    pub(crate) fn new(lex: Lexer<'source, TokenKind>) -> Scanner<'source> {
        Scanner {
            inner: lex.spanned()
        }
    }
}

impl<'source> Iterator for Scanner<'source> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let (result, span) = self.inner.next()?;

        Some(result.and_then(|kind| Ok(Token {
            kind,
            span,
            code: self.inner.source()[span].to_string(),
        })).or_else(|mut e| {
            e.span = Some(span);
            Err(e)
        }))
    }
}

fn chr_to_byte(chr: char) -> Result<u8, Error> {
    if !chr.is_ascii() {
        return Err(Error {
            kind: ErrorKind::InvalidForByteLiteral,
            span: None,
            code: None,
            help: Some("Only characters from '\\0' (null) to '\\x7F' (delete) can be present within a byte literal. For characters from '\\x80' to '\\xFF', use a byte escape.".into())
        })
    }

    let mut buf = [0u8; 1];
    chr.encode_utf8(&mut buf);
    Ok(buf[0])
}