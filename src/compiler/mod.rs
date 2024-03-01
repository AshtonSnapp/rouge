//--> Imports & Modules <--

mod parser;

//--> Type Aliases <--

//--> Structs <--

pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error>>,
    file: String,
    notes: Vec<String>,
}

//--> Enums <--

pub enum ErrorKind {
    Io,
    Scanner,
    Parser,
    // Macros,
    Types,
}

//--> Functions & Impls <--