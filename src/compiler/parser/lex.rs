//--> Imports & Modules <--

use std::{
    iter::Iterator,
    num::{
        ParseIntError,
        ParseFloatError,
    },
    str::FromStr,
};

use logos::{
    Logos,
    Span,
    SpannedIter,
};

//--> Type Aliases <--

//--> Structs <--

pub(super) struct Lex<'a> {
    inner: SpannedIter<'a, TokenKind>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub code: String,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterpolatedStringPart {
    pub text: String,
    pub kind: PartKind,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub code: Option<String>,
    pub span: Option<Span>,
    pub notes: Vec<String>,
}

//--> Enums <--

///
#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\f]+")]
#[logos(error = Error)]
pub enum TokenKind {

    // Literal Tokens - Contain Data

    /// A UTF-8 character literal.
    #[regex(r"'([^']|\\')'", callback = |lex| {
        
        let stripped_slice = lex.slice()
            .strip_prefix('\'').unwrap()
            .strip_suffix('\'').unwrap();

        if stripped_slice.is_empty() {
            return Err(Error {
                kind: ErrorKind::EmptyCharacterOrByteLiteral,
                code: None,
                span: Some(1..lex.slice().len() - 1),
                notes: vec![],
            })
        }

        let chars: Vec<char> = stripped_slice.chars()
            .collect();

        if chars[0] == '\\' {

            match chars.len() {
                1 => Err(Error {
                    kind: ErrorKind::EscapeSequenceTooShort,
                    code: None,
                    span: Some(1..2),
                    notes: vec![],
                }),
                2 => match chars[1] {
                    '\\' => Ok('\\'),
                    '\'' => Ok('\''),
                    '"' => Ok('"'),
                    '0' => Ok('\0'),
                    't' => Ok('\t'),
                    'n' => Ok('\n'),
                    'r' => Ok('\r'),
                    'x' => Err(Error {
                        kind: ErrorKind::EscapeSequenceTooShort,
                        code: None,
                        span: Some(1..3),
                        notes: vec!["ASCII escape sequences must be exactly four characters long: '\\xHH' where H is a hexidecimal digit".into()],
                    }),
                    'u' => Err(Error {
                        kind: ErrorKind::EscapeSequenceTooShort,
                        code: None,
                        span: Some(1..3),
                        notes: vec!["UTF-8 escape sequences must be five to ten characters long: '\\u{H*}' where H* is 1 to 6 hexadecimal digits".into()],
                    }),
                    chr => Ok(chr),
                },
                4 if chars[1] == 'x' => {
                    
                    let mut value = String::new();
                    value.push(chars[2]);
                    value.push(chars[3]);

                    match u8::from_str_radix(&value, 16) {
                        Ok(byte) if byte.is_ascii() => Ok(char::from_u32(byte as u32).unwrap()),
                        Ok(_) => Err(Error {
                            kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(None),
                            code: None,
                            span: Some(3..4),
                            notes: vec![
                                "The last two characters of an ASCII escape sequence must be hexadecimal digits equalling the value of an ASCII character, which is within the range from 0 to 127, or 00 to 7F in hexadecimal.".into()
                            ],
                        }),
                        Err(e) => Err(Error {
                            kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                            code: None,
                            span: Some(3..4),
                            notes: vec![],
                        }),
                    }

                },
                5..=10 if chars[1] == 'u' => {
                    
                    if chars[2] != '{' {
                        return Err(Error {
                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                            code: None,
                            span: Some(2..3),
                            notes: vec![
                                "UTF-8 escape sequences have braces surrounding their value, e.g. '\\u{2660}' for the character '♠'.\
                                You are missing the opening brace.".into()],
                        })
                    } else if chars[chars.len() - 1] != '}' {
                        return Err(Error {
                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                            code: None,
                            span: Some(chars.len()..chars.len() + 1),
                            notes: vec![
                                "UTF-8 escape sequences have braces surrounding their value, e.g. '\\u{2660}' for the character '♠'.\
                                You are missing the closing brace.".into()],
                        })
                    }

                    let mut value = String::new();
                    for chr in &chars[3..chars.len() - 1] { value.push(*chr); }

                    match u32::from_str_radix(&value, 16) {
                        Ok(val) => char::from_u32(val).ok_or_else(|| Error {
                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                            code: None,
                            span: Some(3..chars.len()),
                            notes: vec![],
                        }),
                        Err(e) => Err(Error {
                            kind: ErrorKind::InvalidUtf8EscapeSequence(Some(e)),
                            code: None,
                            span: Some(3..chars.len()),
                            notes: vec![],
                        }),
                    }
                },
                _ if chars[1] == 'x' => Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(5..chars.len()),
                    notes: vec![],
                }),
                _ if chars[1] == 'u' => Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(12..chars.len()),
                    notes: vec![],
                }),
                _ => Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(3..chars.len()),
                    notes: vec![],
                }),
            }

        } else {

            if chars.len() != 1 {
                return Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(2..chars.len()),
                    notes: vec![],
                })
            }

            Ok(chars[0])

        }
    })]
    CharacterLiteral(char),
    /// A UTF-8 string literal. May be cooked (escapes processed) or raw (escapes not processed).
    #[regex(r#""([^"]|\\")*""#, callback = |lex| {
        
        let stripped_slice = lex.slice()
            .strip_prefix('"').unwrap()
            .strip_suffix('"').unwrap();

        if stripped_slice.is_empty() { return Ok("".into()) }

        let mut chars: Vec<char> = stripped_slice.chars().collect();
        let mut output = String::new();
        let mut errors: Vec<Error> = Vec::new();
        let mut current = 0;

        'main: loop {
            let chr0 = chars.pop().unwrap();
            let start = current;
            current += 1;

            if chr0 == '\\' {
                match chars.pop() {
                    Some('\\') => { current += 1; output.push('\\'); },
                    Some('\'') => { current += 1; output.push('\''); },
                    Some('"') => { current += 1; output.push('"'); },
                    Some('0') => { current += 1; output.push('\0'); },
                    Some('t') => { current += 1; output.push('\t'); },
                    Some('n') => { current += 1; output.push('\n'); },
                    Some('r') => { current += 1; output.push('\r'); },
                    Some('x') => {

                        // ASCII Escape Sequence

                        current += 1;
                        let mut value = String::new();

                        for _ in 0..2 {
                            match chars.pop() {
                                Some(chr) => value.push(chr),
                                None => {
                                    errors.push(Error {
                                        kind: ErrorKind::EscapeSequenceTooShort,
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![]
                                    });
                                    break 'main;
                                }
                            }
                            current += 1;
                        }

                        match u8::from_str_radix(&value, 16) {
                            Ok(byte) if byte.is_ascii() => output.push(char::from_u32(byte as u32).unwrap()),
                            Ok(_) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(None),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            }),
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            })
                        }

                    },
                    Some('u') => {

                        // UTF-8 Escape Sequence

                        current += 1;
                        match chars.pop() {
                            Some('{') => {
                                current += 1;
                                let mut value = String::new();

                                loop {
                                    match chars.pop() {
                                        Some('}') => {
                                            current += 1;
                                            break;
                                        },
                                        Some(chr) => {
                                            current += 1;
                                            value.push(chr);
                                        },
                                        None => {
                                            errors.push(Error {
                                                kind: ErrorKind::EscapeSequenceTooShort,
                                                code: None,
                                                span: Some(start..current),
                                                notes: vec![]
                                            });
                                            break 'main;
                                        }
                                    }
                                }

                                match u32::from_str_radix(&value, 16) {
                                    Ok(num) => match char::from_u32(num) {
                                        Some(chr) => output.push(chr),
                                        None => errors.push(Error {
                                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                            code: None,
                                            span: Some(start..current),
                                            notes: vec![],
                                        })
                                    },
                                    Err(e) => errors.push(Error {
                                        kind: ErrorKind::InvalidUtf8EscapeSequence(Some(e)),
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![],
                                    })
                                }

                            },
                            Some(_) => {
                                current += 1;
                                errors.push(Error {
                                    kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                            },
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::EscapeSequenceTooShort,
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                                break 'main;
                            },
                        }

                    },
                    Some(chr) => output.push(chr),
                    None => {
                        errors.push(Error {
                            kind: ErrorKind::EscapeSequenceTooShort,
                            code: None,
                            span: Some(start..current),
                            notes: vec![]
                        });
                        break 'main;
                    },
                }
            } else {
                output.push(chr0);
            }
            
            if chars.is_empty() {
                break 'main;
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors[0].clone());
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                code: None,
                span: None,
                notes: vec![],
            });
        }

        Ok(output)

    })]
    #[regex(r##"r#"[^("#r)]"#r"##, callback = |lex| lex.slice().strip_prefix("r#\"").unwrap().strip_suffix("\"#r").unwrap().to_string())]
    StringLiteral(String),
    /// Part of an interpolated UTF-8 string literal.
    #[regex(r#""([^(\\\{)]|\\")*\\\{"#, callback = |lex| {

        let stripped_slice = lex.slice()
            .strip_prefix('"').unwrap()
            .strip_suffix("\\{").unwrap();

        if stripped_slice.is_empty() { return Ok(InterpolatedStringPart { text: "".into(), kind: PartKind::Initial }) }

        let mut chars: Vec<char> = stripped_slice.chars().collect();
        let mut output = String::new();
        let mut errors: Vec<Error> = Vec::new();
        let mut current = 0;

        'main: loop {
            let chr0 = chars.pop().unwrap();
            let start = current;
            current += 1;

            if chr0 == '\\' {
                match chars.pop() {
                    Some('\\') => { current += 1; output.push('\\'); },
                    Some('\'') => { current += 1; output.push('\''); },
                    Some('"') => { current += 1; output.push('"'); },
                    Some('0') => { current += 1; output.push('\0'); },
                    Some('t') => { current += 1; output.push('\t'); },
                    Some('n') => { current += 1; output.push('\n'); },
                    Some('r') => { current += 1; output.push('\r'); },
                    Some('x') => {

                        // ASCII Escape Sequence

                        current += 1;
                        let mut value = String::new();

                        for _ in 0..2 {
                            match chars.pop() {
                                Some(chr) => value.push(chr),
                                None => {
                                    errors.push(Error {
                                        kind: ErrorKind::EscapeSequenceTooShort,
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![]
                                    });
                                    break 'main;
                                }
                            }
                            current += 1;
                        }

                        match u8::from_str_radix(&value, 16) {
                            Ok(byte) if byte.is_ascii() => output.push(char::from_u32(byte as u32).unwrap()),
                            Ok(_) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(None),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            }),
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            })
                        }

                    },
                    Some('u') => {

                        // UTF-8 Escape Sequence

                        current += 1;
                        match chars.pop() {
                            Some('{') => {
                                current += 1;
                                let mut value = String::new();

                                loop {
                                    match chars.pop() {
                                        Some('}') => {
                                            current += 1;
                                            break;
                                        },
                                        Some(chr) => {
                                            current += 1;
                                            value.push(chr);
                                        },
                                        None => {
                                            errors.push(Error {
                                                kind: ErrorKind::EscapeSequenceTooShort,
                                                code: None,
                                                span: Some(start..current),
                                                notes: vec![]
                                            });
                                            break 'main;
                                        }
                                    }
                                }

                                match u32::from_str_radix(&value, 16) {
                                    Ok(num) => match char::from_u32(num) {
                                        Some(chr) => output.push(chr),
                                        None => errors.push(Error {
                                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                            code: None,
                                            span: Some(start..current),
                                            notes: vec![],
                                        })
                                    },
                                    Err(e) => errors.push(Error {
                                        kind: ErrorKind::InvalidUtf8EscapeSequence(Some(e)),
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![],
                                    })
                                }

                            },
                            Some(_) => {
                                current += 1;
                                errors.push(Error {
                                    kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                            },
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::EscapeSequenceTooShort,
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                                break 'main;
                            },
                        }

                    },
                    Some(chr) => output.push(chr),
                    None => {
                        errors.push(Error {
                            kind: ErrorKind::EscapeSequenceTooShort,
                            code: None,
                            span: Some(start..current),
                            notes: vec![]
                        });
                        break 'main;
                    },
                }
            } else {
                output.push(chr0);
            }
            
            if chars.is_empty() {
                break 'main;
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors[0].clone())
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                code: None,
                span: None,
                notes: vec![]
            })
        }

        Ok(InterpolatedStringPart { text: output, kind: PartKind::Initial })

    })]
    #[regex(r#"\}([^(\\\{)]|\\")*\\\{"#, callback = |lex| {

        let stripped_slice = lex.slice()
            .strip_prefix('}').unwrap()
            .strip_suffix("\\{").unwrap();

        if stripped_slice.is_empty() { return Ok(InterpolatedStringPart { text: "".into(), kind: PartKind::Medial }) }

        let mut chars: Vec<char> = stripped_slice.chars().collect();
        let mut output = String::new();
        let mut errors: Vec<Error> = Vec::new();
        let mut current = 0;

        'main: loop {
            let chr0 = chars.pop().unwrap();
            let start = current;
            current += 1;

            if chr0 == '\\' {
                match chars.pop() {
                    Some('\\') => { current += 1; output.push('\\'); },
                    Some('\'') => { current += 1; output.push('\''); },
                    Some('"') => { current += 1; output.push('"'); },
                    Some('0') => { current += 1; output.push('\0'); },
                    Some('t') => { current += 1; output.push('\t'); },
                    Some('n') => { current += 1; output.push('\n'); },
                    Some('r') => { current += 1; output.push('\r'); },
                    Some('x') => {

                        // ASCII Escape Sequence

                        current += 1;
                        let mut value = String::new();

                        for _ in 0..2 {
                            match chars.pop() {
                                Some(chr) => value.push(chr),
                                None => {
                                    errors.push(Error {
                                        kind: ErrorKind::EscapeSequenceTooShort,
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![]
                                    });
                                    break 'main;
                                }
                            }
                            current += 1;
                        }

                        match u8::from_str_radix(&value, 16) {
                            Ok(byte) if byte.is_ascii() => output.push(char::from_u32(byte as u32).unwrap()),
                            Ok(_) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(None),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            }),
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            })
                        }

                    },
                    Some('u') => {

                        // UTF-8 Escape Sequence

                        current += 1;
                        match chars.pop() {
                            Some('{') => {
                                current += 1;
                                let mut value = String::new();

                                loop {
                                    match chars.pop() {
                                        Some('}') => {
                                            current += 1;
                                            break;
                                        },
                                        Some(chr) => {
                                            current += 1;
                                            value.push(chr);
                                        },
                                        None => {
                                            errors.push(Error {
                                                kind: ErrorKind::EscapeSequenceTooShort,
                                                code: None,
                                                span: Some(start..current),
                                                notes: vec![]
                                            });
                                            break 'main;
                                        }
                                    }
                                }

                                match u32::from_str_radix(&value, 16) {
                                    Ok(num) => match char::from_u32(num) {
                                        Some(chr) => output.push(chr),
                                        None => errors.push(Error {
                                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                            code: None,
                                            span: Some(start..current),
                                            notes: vec![],
                                        })
                                    },
                                    Err(e) => errors.push(Error {
                                        kind: ErrorKind::InvalidUtf8EscapeSequence(Some(e)),
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![],
                                    })
                                }

                            },
                            Some(_) => {
                                current += 1;
                                errors.push(Error {
                                    kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                            },
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::EscapeSequenceTooShort,
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                                break 'main;
                            },
                        }

                    },
                    Some(chr) => output.push(chr),
                    None => {
                        errors.push(Error {
                            kind: ErrorKind::EscapeSequenceTooShort,
                            code: None,
                            span: Some(start..current),
                            notes: vec![]
                        });
                        break 'main;
                    },
                }
            } else {
                output.push(chr0);
            }
            
            if chars.is_empty() {
                break 'main;
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors[0].clone())
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                code: None,
                span: None,
                notes: vec![]
            })
        }

        Ok(InterpolatedStringPart { text: output, kind: PartKind::Medial })

    })]
    #[regex(r#"\}([^"]|\\")*""#, callback = |lex| {

        let stripped_slice = lex.slice()
            .strip_prefix('}').unwrap()
            .strip_suffix('"').unwrap();

        if stripped_slice.is_empty() { return Ok(InterpolatedStringPart { text: "".into(), kind: PartKind::Final }) }

        let mut chars: Vec<char> = stripped_slice.chars().collect();
        let mut output = String::new();
        let mut errors: Vec<Error> = Vec::new();
        let mut current = 0;

        'main: loop {
            let chr0 = chars.pop().unwrap();
            let start = current;
            current += 1;

            if chr0 == '\\' {
                match chars.pop() {
                    Some('\\') => { current += 1; output.push('\\'); },
                    Some('\'') => { current += 1; output.push('\''); },
                    Some('"') => { current += 1; output.push('"'); },
                    Some('0') => { current += 1; output.push('\0'); },
                    Some('t') => { current += 1; output.push('\t'); },
                    Some('n') => { current += 1; output.push('\n'); },
                    Some('r') => { current += 1; output.push('\r'); },
                    Some('x') => {

                        // ASCII Escape Sequence

                        current += 1;
                        let mut value = String::new();

                        for _ in 0..2 {
                            match chars.pop() {
                                Some(chr) => value.push(chr),
                                None => {
                                    errors.push(Error {
                                        kind: ErrorKind::EscapeSequenceTooShort,
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![]
                                    });
                                    break 'main;
                                }
                            }
                            current += 1;
                        }

                        match u8::from_str_radix(&value, 16) {
                            Ok(byte) if byte.is_ascii() => output.push(char::from_u32(byte as u32).unwrap()),
                            Ok(_) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(None),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            }),
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            })
                        }

                    },
                    Some('u') => {

                        // UTF-8 Escape Sequence

                        current += 1;
                        match chars.pop() {
                            Some('{') => {
                                current += 1;
                                let mut value = String::new();

                                loop {
                                    match chars.pop() {
                                        Some('}') => {
                                            current += 1;
                                            break;
                                        },
                                        Some(chr) => {
                                            current += 1;
                                            value.push(chr);
                                        },
                                        None => {
                                            errors.push(Error {
                                                kind: ErrorKind::EscapeSequenceTooShort,
                                                code: None,
                                                span: Some(start..current),
                                                notes: vec![]
                                            });
                                            break 'main;
                                        }
                                    }
                                }

                                match u32::from_str_radix(&value, 16) {
                                    Ok(num) => match char::from_u32(num) {
                                        Some(chr) => output.push(chr),
                                        None => errors.push(Error {
                                            kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                            code: None,
                                            span: Some(start..current),
                                            notes: vec![],
                                        })
                                    },
                                    Err(e) => errors.push(Error {
                                        kind: ErrorKind::InvalidUtf8EscapeSequence(Some(e)),
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![],
                                    })
                                }

                            },
                            Some(_) => {
                                current += 1;
                                errors.push(Error {
                                    kind: ErrorKind::InvalidUtf8EscapeSequence(None),
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                            },
                            None => {
                                errors.push(Error {
                                    kind: ErrorKind::EscapeSequenceTooShort,
                                    code: None,
                                    span: Some(start..current),
                                    notes: vec![]
                                });
                                break 'main;
                            },
                        }

                    },
                    Some(chr) => output.push(chr),
                    None => {
                        errors.push(Error {
                            kind: ErrorKind::EscapeSequenceTooShort,
                            code: None,
                            span: Some(start..current),
                            notes: vec![]
                        });
                        break 'main;
                    },
                }
            } else {
                output.push(chr0);
            }
            
            if chars.is_empty() {
                break 'main;
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors[0].clone())
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                code: None,
                span: None,
                notes: vec![]
            })
        }

        Ok(InterpolatedStringPart { text: output, kind: PartKind::Final })

    })]
    InterpolatedStringLiteralPart(InterpolatedStringPart),
    /// A byte literal.
    #[regex(r"b'([^']\\')'b", callback = |lex| {
        
        let stripped_slice = lex.slice()
            .strip_prefix('\'').unwrap()
            .strip_suffix('\'').unwrap();

        if stripped_slice.is_empty() {
            return Err(Error {
                kind: ErrorKind::EmptyCharacterOrByteLiteral,
                code: None,
                span: Some(1..lex.slice().len() - 1),
                notes: vec![],
            })
        }

        let chars: Vec<char> = stripped_slice.chars()
            .collect();

        if chars[0] == '\\' {

            match chars.len() {
                1 => Err(Error {
                    kind: ErrorKind::EscapeSequenceTooShort,
                    code: None,
                    span: Some(1..2),
                    notes: vec![],
                }),
                2 => match chars[1] {
                    '\\' => Ok(0x5c),
                    '\'' => Ok(0x27),
                    '"' => Ok(0x22),
                    '0' => Ok(0x00),
                    't' => Ok(0x09),
                    'n' => Ok(0x0a),
                    'r' => Ok(0x0d),
                    'x' => Err(Error {
                        kind: ErrorKind::EscapeSequenceTooShort,
                        code: None,
                        span: Some(1..3),
                        notes: vec!["Byte escape sequences must be exactly four characters long: '\\xHH' where H is a hexidecimal digit".into()],
                    }),
                    chr => if chr.is_ascii() {
                        let mut buf = [0x0u8; 1];
                        chr.encode_utf8(&mut buf);
                        Ok(buf[0])
                    } else {
                        Err(Error {
                            kind: ErrorKind::NonAsciiCharacterInByteLiteral,
                            code: None,
                            span: Some(2..3),
                            notes: vec![]
                        })
                    },
                },
                4 if chars[1] == 'x' => {
                    
                    // Byte Escape Sequence

                    let mut value = String::new();
                    value.push(chars[2]);
                    value.push(chars[3]);

                    match u8::from_str_radix(&value, 16) {
                        Ok(byte) => Ok(byte),
                        Err(e) => Err(Error {
                            kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                            code: None,
                            span: Some(3..4),
                            notes: vec![],
                        }),
                    }

                },
                _ if chars[1] == 'x' => Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(5..chars.len()),
                    notes: vec![],
                }),
                _ => Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(3..chars.len()),
                    notes: vec![],
                }),
            }

        } else {

            if chars.len() != 1 {
                return Err(Error {
                    kind: ErrorKind::CharacterOrByteLiteralOverflow,
                    code: None,
                    span: Some(2..chars.len()),
                    notes: vec![],
                })
            }

            if !chars[0].is_ascii() {
                return Err(Error {
                    kind: ErrorKind::NonAsciiCharacterInByteLiteral,
                    code: None,
                    span: Some(1..2),
                    notes: vec![]
                })
            }

            let mut buf = [0x0u8; 1];
            chars[0].encode_utf8(&mut buf);
            Ok(buf[0])

        }
    })]
    ByteLiteral(u8),
    /// A byte string literal. May be cooked (escapes processed) or raw (escapes not processed).
    #[regex(r#"b"([^"]|\\")*"b"#, callback = |lex| {
        let stripped_slice = lex.slice()
            .strip_prefix("b\"").unwrap()
            .strip_suffix("\"b").unwrap();

        if stripped_slice.is_empty() { return Ok(vec![]) }

        let mut chars: Vec<char> = stripped_slice.chars().collect();
        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();
        let mut current = 0;

        'main: loop {
            let chr0 = chars.pop().unwrap();
            let start = current;
            current += 1;

            if chr0 == '\\' {
                match chars.pop() {
                    Some('\\') => { current += 1; output.push(0x5c); },
                    Some('\'') => { current += 1; output.push(0x27); },
                    Some('"') => { current += 1; output.push(0x22); },
                    Some('0') => { current += 1; output.push(0x00); },
                    Some('t') => { current += 1; output.push(0x09); },
                    Some('n') => { current += 1; output.push(0x0a); },
                    Some('r') => { current += 1; output.push(0x0d); },
                    Some('x') => {

                        // ASCII Escape Sequence

                        current += 1;
                        let mut value = String::new();

                        for _ in 0..2 {
                            match chars.pop() {
                                Some(chr) => value.push(chr),
                                None => {
                                    errors.push(Error {
                                        kind: ErrorKind::EscapeSequenceTooShort,
                                        code: None,
                                        span: Some(start..current),
                                        notes: vec![]
                                    });
                                    break 'main;
                                }
                            }
                            current += 1;
                        }

                        match u8::from_str_radix(&value, 16) {
                            Ok(byte) => output.push(byte),
                            Err(e) => errors.push(Error {
                                kind: ErrorKind::InvalidAsciiOrByteEscapeSequence(Some(e)),
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            })
                        }

                    },
                    Some(chr) => {
                        current +=1;

                        if !chr.is_ascii() {
                            errors.push(Error {
                                kind: ErrorKind::NonAsciiCharacterInByteLiteral,
                                code: None,
                                span: Some(start..current),
                                notes: vec![]
                            })
                        } else {
                            let mut buf = [0x0u8; 1];
                            chr.encode_utf8(&mut buf);
                            output.push(buf[0]);
                        }
                    },
                    None => {
                        errors.push(Error {
                            kind: ErrorKind::EscapeSequenceTooShort,
                            code: None,
                            span: Some(start..current),
                            notes: vec![]
                        });
                        break 'main;
                    },
                }
            } else {
                if !chr0.is_ascii() {
                    errors.push(Error {
                        kind: ErrorKind::NonAsciiCharacterInByteLiteral,
                        code: None,
                        span: Some(start..current),
                        notes: vec![]
                    })
                } else {
                    let mut buf = [0x0u8; 1];
                    chr0.encode_utf8(&mut buf);
                    output.push(buf[0]);
                }
            }
            
            if chars.is_empty() {
                break 'main;
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors[0].clone())
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                code: None,
                span: None,
                notes: vec![]
            })
        }

        Ok(output)
    })]
    #[regex(r##"br#"[^("#rb)]"#rb"##, callback = |lex| {
        let stripped_slice = lex.slice()
            .strip_prefix("br#\"").unwrap()
            .strip_suffix("\"#rb").unwrap();

        if stripped_slice.is_empty() { return Ok(vec![]) }

        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();
        let mut current = 0;

        for chr in stripped_slice.chars() {
            let start = current;
            current += 1;
            if !chr.is_ascii() {
                errors.push(Error {
                    kind: ErrorKind::NonAsciiCharacterInByteLiteral,
                    code: None,
                    span: Some(start..current),
                    notes: vec![]
                })
            } else {
                let mut buf = [0x0u8; 1];
                chr.encode_utf8(&mut buf);
                output.push(buf[0]);
            }
        }

        if !errors.is_empty() {
            if errors.len() == 1 {
                return Err(errors[0].clone())
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                code: None,
                span: None,
                notes: vec![]
            })
        }

        Ok(output)
    })]
    ByteStringLiteral(Vec<u8>),
    /// A 64-bit whole number literal. May be in binary, octal, decimal, or hexadecimal.
    #[regex(r"0[bB][01][01_]*", callback = |lex| u64::from_str_radix(lex.slice().to_lowercase().strip_prefix("0b").unwrap(), 2).map_err(|e| Error {
        kind: ErrorKind::InvalidNumber(e),
        code: None,
        span: None,
        notes: vec![]
    }))]
    #[regex(r"0[oO][0-7][0-7_]*", callback = |lex| u64::from_str_radix(lex.slice().to_lowercase().strip_prefix("0o").unwrap(), 8).map_err(|e| Error {
        kind: ErrorKind::InvalidNumber(e),
        code: None,
        span: None,
        notes: vec![]
    }))]
    #[regex(r"[0-9][0-9_]*", callback = |lex| u64::from_str(lex.slice()).map_err(|e| Error {
        kind: ErrorKind::InvalidNumber(e),
        code: None,
        span: None,
        notes: vec![]
    }))]
    #[regex(r"0[xX][0-9a-fA-F][0-9a-fA-F_]*", callback = |lex| u64::from_str_radix(lex.slice().to_lowercase().strip_prefix("0x").unwrap(), 16).map_err(|e| Error {
        kind: ErrorKind::InvalidNumber(e),
        code: None,
        span: None,
        notes: vec![]
    }))]
    NumberLiteral(u64),
    /// An IEEE 754 double-precision floating point literal. Must be in decimal.
    #[regex(r"[0-9][0-9_]*((.[0-9][0-9_]*)|([eE][\+\-][0-9][0-9_]*)|(.[0-9][0-9_]*[eE][\+\-][0-9][0-9_]*))", callback = |lex| f64::from_str(lex.slice()).map_err(|e| Error {
        kind: ErrorKind::InvalidFloat(e),
        code: None,
        span: None,
        notes: vec![],
    }))]
    FloatLiteral(f64),

    // Symbol Tokens - Operators and Et Cetera

    ///
    #[token("+")]
    PlusSymbol,
    ///
    #[token("-")]
    DashSymbol,
    ///
    #[token("*")]
    StarSymbol,
    ///
    #[token("/")]
    SlashSymbol,
    ///
    #[token("%")]
    PercentSymbol,
    ///
    #[token("%%")]
    DoublePercentSymbol,
    ///
    #[token("&")]
    AndSymbol,
    ///
    #[token("&&")]
    DoubleAndSymbol,
    ///
    #[token("|")]
    PipeSymbol,
    ///
    #[token("||")]
    DoublePipeSymbol,
    ///
    #[token("^")]
    CaretSymbol,
    ///
    #[token("!")]
    BangSymbol,
    ///
    #[token("?")]
    QuerySymbol,
    ///
    #[token("`")]
    TickSymbol,
    ///
    #[token(",")]
    CommaSymbol,
    ///
    #[token(".")]
    DotSymbol,
    ///
    #[token("..")]
    DoubleDotSymbol,
    ///
    #[token(";")]
    SemicolonSymbol,
    ///
    #[token(":")]
    ColonSymbol,
    ///
    #[token("::")]
    DoubleColonSymbol,
    ///
    #[token(".:")]
    DotColonSymbol,
    ///
    #[token("<")]
    LeftChevronSymbol,
    ///
    #[token("-<")]
    VacuumSymbol,
    ///
    #[token("<<")]
    DoubleLeftChevronSymbol,
    ///
    #[token(">")]
    RightChevronSymbol,
    ///
    #[token("->")]
    ArrowSymbol,
    ///
    #[token(">>")]
    DoubleRightChevronSymbol,
    ///
    #[token(">->")]
    FletchedArrowSymbol,
    ///
    #[token("=")]
    EqualSymbol,
    ///
    #[token("==")]
    DoubleEqualSymbol,
    ///
    #[token("+=")]
    PlusEqualSymbol,
    ///
    #[token("-=")]
    DashEqualSymbol,
    ///
    #[token("*=")]
    StarEqualSymbol,
    ///
    #[token("/=")]
    SlashEqualSymbol,
    ///
    #[token("%=")]
    PercentEqualSymbol,
    ///
    #[token("&=")]
    AndEqualSymbol,
    ///
    #[token("|=")]
    PipeEqualSymbol,
    ///
    #[token("^=")]
    CaretEqualSymbol,
    ///
    #[token("!=")]
    BangEqualSymbol,
    ///
    #[token(".=")]
    CyclopsWalrusSymbol,
    ///
    #[token(":=")]
    WalrusSymbol,
    ///
    #[token("<=")]
    LeftChevronEqualSymbol,
    ///
    #[token("<<=")]
    DoubleLeftChevronEqualSymbol,
    ///
    #[token(">=")]
    RightChevronEqualSymbol,
    ///
    #[token(">>=")]
    DoubleRightChevronEqualSymbol,
    ///
    #[token("(")]
    OpenParenSymbol,
    ///
    #[token(")")]
    CloseParenSymbol,
    ///
    #[token("[")]
    OpenBracketSymbol,
    ///
    #[token("]")]
    CloseBracketSymbol,
    ///
    #[token("{")]
    OpenBraceSymbol,
    ///
    #[token("}")]
    CloseBraceSymbol,
    ///
    #[token("@")]
    AtSymbol,
    ///
    #[token("_")]
    UnderscoreSymbol,
    ///
    #[regex(r"(\n|\r|\r\n)")]
    NewlineSymbol,
    ///
    #[regex(r"[!$%^&*\-=\+|;:<>./?]+", callback = |lex| lex.slice().to_string())]
    CustomSymbol(String),

    // Word Tokens - Keywords and Identifiers

    /// Represents a true/false value.
    /// Rather than having different token types for true and false,
    /// why not have a single token type and let it hold either a true or false value?
    #[token("true", callback = |_lex| true)]
    #[token("false", callback = |_lex| false)]
    BooleanValueWord(bool),
    /// The boolean type, represented with the keyword `bool`.
    #[token("bool")]
    BooleanTypeWord,
    /// The number type, represented with the keyword `num`.
    #[token("num")]
    NumberTypeWord,
    /// The natural number type, represented with the keyword `nat`.
    #[token("nat")]
    NaturalNumberTypeWord,
    /// The 8-bit natural number type, represented with the keyword `nat8` OR `byte`.
    #[token("nat8")]
    #[token("byte")]
    NaturalNumber8BitTypeWord,
    /// The 16-bit natural number type, represented with the keyword `nat16`.
    #[token("nat16")]
    NaturalNumber16BitTypeWord,
    /// The 32-bit natural number type, represented with the keyword `nat32`.
    #[token("nat32")]
    NaturalNumber32BitTypeWord,
    /// The 64-bit natural number type, represented with the keyword `nat64`.
    #[token("nat64")]
    NaturalNumber64BitTypeWord,
    /// The integer number type, represented with the keyword `int`.
    #[token("int")]
    IntegerTypeWord,
    /// The 8-bit integer number type, represented with the keyword `int8`.
    #[token("int8")]
    Integer8BitTypeWord,
    /// The 16-bit integer number type, represented with the keyword `int16`.
    #[token("int16")]
    Integer16BitTypeWord,
    /// The 32-bit integer number type, represented with the keyword `int32`.
    #[token("int32")]
    Integer32BitTypeWord,
    /// The 64-bit integer number type, represented with the keyword `int64`.
    #[token("int64")]
    Integer64BitTypeWord,
    /// The IEEE 754 floating point number type, represented with the keyword `flo`.
    #[token("flo")]
    FloatingPointTypeWord,
    /// The IEEE 754 single-precision floating point number type, represented with the keyword `flo32`.
    #[token("flo32")]
    FloatingPoint32BitTypeWord,
    /// The IEEE 754 double-precision floating point number type, represented with the keyword `flo64`.
    #[token("flo64")]
    FloatingPoint64BitTypeWord,
    /// The UTF-8 character type, represented with the keyword `char`.
    #[token("char")]
    CharacterTypeWord,
    /// The UTF-8 string type, represented with the keyword `str`.
    #[token("str")]
    StringTypeWord,
    /// The dynamic type, represented with the keyword `any`. Can be used alone or with a trait bound.
    #[token("any")]
    AnyTypeWord,
    /// The type that a function or method is associated with. Represented with the keyword `Self`.
    #[token("Self")]
    SelfTypeWord,
    /// A reference to an instance of the type the method belongs to. Represented with the keyword `self`.
    #[token("self")]
    SelfWord,
    /// A function definition or alias, represented with the keyword `func`.
    #[token("func")]
    FunctionWord,
    /// A type definition or alias, represented with the keyword `type`.
    #[token("type")]
    TypeWord,
    /// An implementation block, represented with the keyword `impl`.
    #[token("impl")]
    ImplementWord,
    /// A trait definition, represented with the keyword `trait`.
    #[token("trait")]
    TraitWord,
    /// An effect definition, represented with the keyword `effect`.
    #[token("effect")]
    EffectWord,
    ///// A macro definition, represented with the keyword `macro`.
    //#[token("macro")]
    //MacroWord,
    /// Indicates that a definition is, in fact, an alias of another definition.
    /// Specifically, functions, types, traits, and effects can be aliased.
    #[token("alias")]
    AliasWord,
    /// Specifies an item as being public (accessible outside of the package), represented with the keyword `pub`.
    #[token("pub")]
    PublicWord,
    /// Specifies an item as being protected (only accessible within a given scope within the package), represented with the keyword `prt`.
    #[token("prt")]
    ProtectedWord,
    /// Specifies a mutable variable, as opposed to an immutable variable, represented with the keyword `mut`.
    #[token("mut")]
    MutableWord,
    /// Specifies a constant, which is immutable and must be known at compile time.
    /// 
    /// Can also mark a function as constant, meaning it can be used in a constant context. Constant functions cannot have effects or mutability.
    #[token("const")]
    ConstantWord,
    /// First branch of a conditional branching block.
    #[token("if")]
    IfWord,
    /// Nth branch of a conditional branching block.
    #[token("elif")]
    ElseIfWord,
    /// Final branch of a conditional branching block.
    #[token("else")]
    ElseWord,
    /// Operator that performs pattern matching. May bind variables, usually used as part of an `if` block or `while` loop.
    #[token("matches")]
    MatchesWord,
    /// Seperator between the condition or pattern of a conditional/matching block.
    #[token("then")]
    ThenWord,
    /// Declares a loop that goes `always`, unless you break out of it or crash.
    #[token("always")]
    AlwaysWord,
    /// Declares a loop that goes `while` a condition is true. May take one of the following forms:
    /// ```rouge
    /// while condition do
    ///     code
    /// end
    /// ```
    /// or
    /// ```rouge
    /// do
    ///     code
    /// while condition end
    /// ```
    #[token("while")]
    WhileWord,
    /// Declares a loop that goes `for`` each item within some iterator.
    #[token("for")]
    ForWord,
    /// Can be used as an operator to check for membership in an iterable type,
    /// a separator between a variable and an iterable type in a `for` loop,
    /// or as a post-scope for an effect handler.
    #[token("in")]
    InWord,
    /// Can be used as either an immediately called anonymous function,
    /// or as a pre-scope for an effect handler.
    /// Depends on the presence of `do`.
    #[token("with")]
    WithWord,
    /// Used to create an effect handler.
    #[token("when")]
    WhenWord,
    /// Used to place trait bounds on generic types.
    #[token("where")]
    WhereWord,
    /// Used to perform an operation of an effect.
    #[token("perform")]
    PerformWord,
    /// Returns a value from a function. Only required for early returns.
    #[token("return")]
    ReturnWord,
    /// Resumes the function which performed the effectful operation.
    #[token("resume")]
    ResumeWord,
    /// Skips one iteration of a loop.
    #[token("skip")]
    SkipWord,
    /// Breaks out of a loop or labelled block. A block label may be supplied.
    #[token("break")]
    BreakWord,
    /// Used to separate the signatures of code structures from their contents.
    #[token("do")]
    DoWord,
    /// Used to separate the signatures of type, trait, and effect definitions from their contents.
    #[token("is")]
    IsWord,
    /// Uses an item or module from a package, as in `use std::io`. `use std` uses the root module of `std`.
    #[token("use")]
    UseWord,
    /// Aliases a used item or module, as in `use std::io::Error as IoError`. Also used to separate the signature of an impl block from its contents.
    #[token("as")]
    AsWord,
    /// Ends a block.
    #[token("end")]
    EndWord,
    /// Logical and.
    #[token("and")]
    AndWord,
    /// Logical or.
    #[token("or")]
    OrWord,
    ///
    #[regex(r"(\p{XID_Start}|_)(\p{XID_Continue}|_)*", callback = |lex| lex.slice().to_string())]
    IdentifierWord(String),

    // Comment Tokens - These are discarded at some point in the parsing process.

    ///
    #[regex(r"#[^(\n|\r|\r\n)]", callback = |lex| lex.slice().strip_prefix("#").unwrap().to_string())]
    #[regex(r"#\[[^(\]#)]\]#", callback = |lex| lex.slice().strip_prefix("#[").unwrap().strip_suffix("]#").unwrap().to_string())]
    Comment(String),
    ///
    #[regex(r"##[^(\n|\r|\r\n)]", callback = |lex| lex.slice().strip_prefix("##").unwrap().to_string())]
    #[regex(r"##![^(\n|\r|\r\n)]", callback = |lex| lex.slice().strip_prefix("##!").unwrap().to_string())]
    #[regex(r"##\[[^(\]##)]\]##", callback = |lex| lex.slice().strip_prefix("##[").unwrap().strip_suffix("]##").unwrap().to_string())]
    #[regex(r"##!\[[^(\]!##)]\]!##", callback = |lex| lex.slice().strip_prefix("##![").unwrap().strip_suffix("]!##").unwrap().to_string())]
    DocumentationComment(String),

}

#[derive(Clone, Debug, PartialEq)]
pub enum PartKind {
    ///
    Initial,
    ///
    Medial,
    ///
    Final,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ErrorKind {
    EmptyCharacterOrByteLiteral,
    CharacterOrByteLiteralOverflow,
    EscapeSequenceTooShort,
    InvalidAsciiOrByteEscapeSequence(Option<ParseIntError>),
    InvalidUtf8EscapeSequence(Option<ParseIntError>),
    NonAsciiCharacterInByteLiteral,
    InvalidNumber(ParseIntError),
    InvalidFloat(ParseFloatError),
    Multiple(Vec<Error>),
    #[default]
    Other,
}

//--> Functions & Impls <--

impl<'a> Lex<'a> {
    pub fn new(string: &'a str) -> Lex<'a> {
        Lex {
            inner: TokenKind::lexer(string).spanned()
        }
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            (Ok(kind), span) => {
                Some(Ok(Token {
                    kind,
                    code: self.inner.slice().into(),
                    span,
                }))
            },
            (Err(ref mut e), span) => {
                if let ErrorKind::Multiple(ref mut vec) = e.kind {
                    for err in vec {
                        if let Some(rel_span) = &err.span {
                            err.span = Some((rel_span.start + span.clone().start)..(rel_span.end + span.clone().start));
                        } else {
                            err.span = Some(span.clone());
                        }
                    }
                }

                if let Some(ref rel_span) = e.span {
                    e.span = Some((rel_span.start + span.clone().start)..(rel_span.end + span.clone().start));
                } else {
                    e.span = Some(span);
                }
                Some(Err(e.clone()))
            }
        }
    }
}