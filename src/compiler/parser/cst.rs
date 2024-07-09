//--> Imports & Modules <--

use super::scan::{
    Scanner,
    Token,
    TokenKind,
    Error as ScanError,
};

use std::path::PathBuf;

use logos::Span;

//--> Structs <--

pub(crate) struct ConcreteSyntaxTree {
    file: PathBuf,
    root: ConcreteSyntaxNode,
}

pub(crate) struct ConcreteSyntaxNode {
    data: Token,
    left: Option<Box<ConcreteSyntaxNode>>,
    right: Option<Box<ConcreteSyntaxNode>>,
}

#[derive(Clone, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    span: Option<Span>,
    code: Option<String>,
    help: Option<String>,
}

//--> Enums <--

#[derive(Clone, PartialEq)]
pub enum ErrorKind {
    Scanner(ScanError),
    Multiple(Vec<Error>),
}

//--> Functions & Impls <--

impl ConcreteSyntaxTree {
    pub(crate) fn generate<'source>(mut scanner: Scanner<'source>, filename: PathBuf) -> Result<ConcreteSyntaxTree, Error> {
        
        let mut node_stack = Vec::new();
        let mut errors = Vec::new();

        errors.append(scanner.filter_map(|r| if let Err(e) = r { Some(e) } else { None }).collect());
        node_stack.append(scanner.filter_map(|r| if let Ok(tok) = r { Some(tok) } else { None }).collect());

        loop {
            // TODO: ??????
        }

        if !errors.is_empty() {
            
            if errors.len() == 1 {
                return Err(errors.get(0).unwrap().clone());
            }

            return Err(Error {
                kind: ErrorKind::Multiple(errors),
                span: None,
                code: None,
                help: None,
            });
        }

        Ok(ConcreteSyntaxTree {
            file: filename,
            root: node_stack.pop().unwrap(),
        })

    }
}