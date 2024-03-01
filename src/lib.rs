#![feature(box_into_inner)]

//--> Imports & Modules <--

mod compiler;

use std::{
    fmt,
    ops::Deref,
    ops::DerefMut,
};

//--> Type Aliases <--

//--> Structs <--

/// Contains a list of errors, and is itself an error.
/// 
/// Note: In general, lists of errors are only output by the compiler.
/// If you are writing to a native command-line, it is recommended you
/// convert these compiler errors into [`codespan_reporting::diagnostic::Diagnostic`]s
/// by using the `to_diagnostic` function, and then output them to
/// the terminal using [`codespan_reporting::term::emit`].
/// 
/// Otherwise, you can still convert to diagnostic to get location information that is more specific than
/// the byte span the compiler errors contain. This is because the compiler represents everything in byte spans,
/// while you will want row and column positions - which Diagnostics can derive from the byte spans.
#[derive(Debug)]
pub struct Errors<E: std::error::Error>(pub Vec<E>);

//--> Enums <--

//--> Functions & Impls <--

impl<E: std::error::Error> Errors<E> {
    pub(crate) fn new() -> Errors<E> {
        Errors(vec![])
    }
}

impl<E: std::error::Error> Deref for Errors<E> {
    type Target = Vec<E>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E: std::error::Error> DerefMut for Errors<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E: std::error::Error> fmt::Display for Errors<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for err in &self.0 {
            output.push_str(&format!("{}\n\n", err.to_string()));
        }

        output.push_str(&format!("{} total errors encountered.\n", self.0.len()));

        write!(f, "{}", output)
    }
}

impl<E: std::error::Error> std::error::Error for Errors<E> {}