//--> Imports & Modules <--

mod cst;
mod ast;
mod lex;

use lex::{Lex, Error as LexError, ErrorKind as LexErrorKind, TokenKind};

use ast::AbstractSyntaxTree;

use crate::Errors;

use logos::Span;

use std::{
    convert::{
        AsRef,
        From,
    },
    fmt,
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
};

//--> Type Aliases <--

pub type Result<T> = std::result::Result<T, Error>;

//--> Structs <--

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub source: Option<Box<dyn std::error::Error>>,
    pub file: Option<PathBuf>,
    pub code: Option<String>,
    pub span: Option<Span>,
    pub notes: Vec<String>,
}

//--> Enums <--

#[derive(Debug)]
pub enum ErrorKind {
    Io,
    EmptyCharacterOrByteLiteral,
    CharacterOrByteLiteralOverflow,
    EscapeSequenceTooShort,
    InvalidAsciiOrByteEscapeSequence,
    InvalidUtf8EscapeSequence,
    NonAsciiCharacterInByteLiteral,
    InvalidNumber,
    InvalidFloat,
    InvalidToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
    },
    EmptyNodeStackAtEnd,
    Multiple,
    Other,
}

//--> Functions & Impls <--

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(e) => Some(e.as_ref()),
            None => None,
        }
    }
}

impl From<LexError> for Error {
    fn from(value: LexError) -> Error {
        Error {
            kind: match &value.kind {
                LexErrorKind::EmptyCharacterOrByteLiteral => ErrorKind::EmptyCharacterOrByteLiteral,
                LexErrorKind::CharacterOrByteLiteralOverflow => ErrorKind::CharacterOrByteLiteralOverflow,
                LexErrorKind::EscapeSequenceTooShort => ErrorKind::EscapeSequenceTooShort,
                LexErrorKind::InvalidAsciiOrByteEscapeSequence(_) => ErrorKind::InvalidAsciiOrByteEscapeSequence,
                LexErrorKind::InvalidUtf8EscapeSequence(_) => ErrorKind::InvalidUtf8EscapeSequence,
                LexErrorKind::NonAsciiCharacterInByteLiteral => ErrorKind::NonAsciiCharacterInByteLiteral,
                LexErrorKind::InvalidNumber(_) => ErrorKind::InvalidNumber,
                LexErrorKind::InvalidFloat(_) => ErrorKind::InvalidFloat,
                LexErrorKind::Multiple(_) => ErrorKind::Multiple,
                LexErrorKind::Other => ErrorKind::Other,
            },
            source: match value.kind {
                LexErrorKind::InvalidAsciiOrByteEscapeSequence(e) => e.map(|e| e.into()),
                LexErrorKind::InvalidUtf8EscapeSequence(e) => e.map(|e| e.into()),
                LexErrorKind::InvalidNumber(e) => Some(Box::new(e)),
                LexErrorKind::InvalidFloat(e) => Some(Box::new(e)),
                LexErrorKind::Multiple(e) => {
                    let mut errors = Errors::new();

                    for err in e { errors.push(Box::new(Error::from(err))); }

                    Some(Box::new(errors))
                },
                _ => None,
            },
            file: None,
            code: value.code,
            span: value.span,
            notes: value.notes
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error {
            kind: ErrorKind::Io,
            source: Some(Box::new(value)),
            file: None,
            code: None,
            span: None,
            notes: vec![],
        }
    }
}

pub fn parse<P: AsRef<Path>, PS: AsRef<[P]>>(paths: PS) -> Result<AbstractSyntaxTree> {
    let mut errors: Errors<Error> = Errors(vec![]);

    // .as_ref().as_ref() lol
    let paths: Vec<&Path> = paths.as_ref()
        .iter()
        .map(|p| p.as_ref())
        .collect();
    
    let content_results: Vec<Result<String>> = paths.iter()
        .map(|p| {
            fs::read_to_string(p)
                .map_err(|e| Error::from(e))
        })
        .collect();

    if content_results.iter().any(|r| r.is_err()) {
        errors.append(&mut content_results.iter()
            .filter(|r| r.is_err())
            .map(|r| match *r { Err(e) => e, _ => unreachable!() })
            .collect()
        );
    }

    let contents: Vec<String> = content_results.into_iter().filter(|r| r.is_ok()).map(|r| match r {
        Ok(s) => s,
        _ => unreachable!(), 
    }).collect();

    let results: Vec<(Result<AbstractSyntaxTree>, &Path)> = contents.iter()
        .map(|s| match cst::generate(Lex::new(&s)) {
            Ok(node) => ast::generate(node),
            Err(e) => Err(e)
        })
        .zip(paths.into_iter())
        .collect();

    if !errors.is_empty() {
        if errors.len() == 1 {
            return Err(errors[0])
        }

        return Err(Error {
            kind: ErrorKind::Multiple,
            source: Some(Box::new(errors)),
            file: None,
            code: None,
            span: None,
            notes: vec![],
        })
    }

    todo!()
}