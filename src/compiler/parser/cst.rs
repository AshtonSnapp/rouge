//--> Imports & Modules <--

use super::{
    lex::{
        Lex,
        Token,
        TokenKind,
        Error as LexError,
        ErrorKind as LexErrorKind,
    },
    Error,
    ErrorKind,
};

use crate::Errors;

//--> Type Aliases <--

//--> Structs <--

pub(crate) struct ConcreteSyntaxNode {
    token: Token,
    left: Option<Box<ConcreteSyntaxNode>>,
    right: Option<Box<ConcreteSyntaxNode>>,
}

//--> Functions & Impls <--

pub(crate) fn generate<'a>(lex: Lex<'a>) -> Result<ConcreteSyntaxNode, Error> {
    let mut errors = Errors::new();
    let mut node_stack: Vec<ConcreteSyntaxNode> = Vec::new();

    for result in lex {
        match result {
            Ok(tok) => match node_stack.last() {
                Some(node) => match node.token.kind {
                    TokenKind::
                },
                None => match tok.kind {
                    // Comments are skipped, and newlines have no meaning at top level.
                    TokenKind::Comment(_)
                        | TokenKind::DocumentationComment(_)
                        | TokenKind::NewlineSymbol => continue,
                    // `use`, `func`, `type`, `impl`, `trait`, `effect`, and `const` indicate items that can be at top level.
                    // `pub` and `prt` are visibility modifiers.
                    // `@` is used for attributes.
                    TokenKind::UseWord
                        | TokenKind::FunctionWord
                        | TokenKind::TypeWord
                        | TokenKind::ImplementWord
                        | TokenKind::TraitWord
                        | TokenKind::EffectWord
                        | TokenKind::ConstantWord
                        | TokenKind::PublicWord
                        | TokenKind::ProtectedWord
                        | TokenKind::IdentifierWord(_)
                        | TokenKind::AtSymbol => node_stack.push(ConcreteSyntaxNode::new(tok)),
                    // Everything else is straight up invalid.
                    other => errors.push(Error {
                        kind: ErrorKind::InvalidToken { expected: vec![
                            TokenKind::UseWord,
                            TokenKind::FunctionWord,
                            TokenKind::TypeWord,
                            TokenKind::ImplementWord,
                            TokenKind::TraitWord,
                            TokenKind::EffectWord,
                            TokenKind::ConstantWord,
                            TokenKind::PublicWord,
                            TokenKind::ProtectedWord,
                            TokenKind::IdentifierWord(String::default()),
                            TokenKind::AtSymbol,
                            TokenKind::Comment(String::default()), // these token kinds contain data, but we don't care about it.
                            TokenKind::DocumentationComment(String::default()), // these token kinds contain data, but we don't care about it.
                            TokenKind::NewlineSymbol,
                        ], found: other },
                        source: None,
                        file: None,
                        code: Some(tok.code),
                        span: Some(tok.span),
                        notes: vec![],
                    })
                },
            },
            Err(e) => {
                let error = Error::from(e);
                if let ErrorKind::Multiple = error.kind {
                    if let Some(e) = error.source {
                        errors.append(&mut e.downcast::<Errors<Error>>().unwrap());
                    } else {
                        unreachable!()
                    }
                } else {
                    errors.push(error);
                }
            },
        }
    }

    if node_stack.is_empty() {
        errors.push(Error {
            kind: ErrorKind::EmptyNodeStackAtEnd,
            source: None,
            file: None,
            code: None,
            span: None,
            notes: vec![],
        })
    }

    if !errors.is_empty() {
        return Err(Error {
            kind: ErrorKind::Multiple,
            source: Some(Box::new(errors)),
            file: None,
            code: None,
            span: None,
            notes: vec![],
        })
    }

    Ok(node_stack.pop().unwrap())
}

impl ConcreteSyntaxNode {
    pub(crate) fn new(token: Token) -> ConcreteSyntaxNode {
        ConcreteSyntaxNode {
            token,
            left: None,
            right: None,
        }
    }

    pub(crate) fn with_lefthand(self, node: ConcreteSyntaxNode) -> ConcreteSyntaxNode {
        ConcreteSyntaxNode {
            left: Some(Box::new(node)),
            ..self
        }
    }

    pub(crate) fn with_righthand(self, node: ConcreteSyntaxNode) -> ConcreteSyntaxNode {
        ConcreteSyntaxNode {
            right: Some(Box::new(node)),
            ..self
        }
    }

    pub(crate) fn add_lefthand(&mut self, node: ConcreteSyntaxNode) {
        let _ = self.left.insert(Box::new(node));
    }

    pub(crate) fn add_righthand(&mut self, node: ConcreteSyntaxNode) {
        let _ = self.right.insert(Box::new(node));
    }
}