//--> Imports & Modules <--

use super::{
    lex::{
        Token,
        TokenKind,
    },
    cst::ConcreteSyntaxNode,
    Error,
    ErrorKind,
    Result,
};

//--> Type Aliases <--

//--> Structs <--

pub(crate) struct AbstractSyntaxTree {
    items: Vec<Item>,
}

pub(crate) struct Generic {
    name: String,
    trait_bounds: Vec<String>,
}

pub(crate) struct Field {
    name: String,
    type_: Type,
}

pub(crate) struct Variant {
    name: String,
    fields: Vec<Field>,
}

pub(crate) struct FunctionPrototype {
    name: String,
    generics: Vec<Generic>,
    arguments: Vec<Type>,
    return_type: Type,
    effects: Vec<EffectType>,
    has_generic_effects: bool,
}

pub(crate) struct EffectType {
    name: String,
    generics: Vec<Generic>,
}

pub(crate) struct Operation {
    name: String,
    generics: Vec<Generic>,
    arguments: Vec<Type>,
    return_type: Type,
}

//--> Enums <--

pub(crate) enum Item {
    UseTree {
        package: String,
        items: Vec<Item>,
    },
    Module {
        name: String,
        items: Vec<Item>,
    },
    Function {
        name: String,
        generics: Vec<Generic>,
        arguments: Vec<Field>,
        return_type: Type,
        effect_types: Vec<EffectType>,
        has_generic_effects: bool,
    },
    Type {
        name: String,
        fields: Vec<Field>,
        variants: Vec<Variant>,
    },
    ImplementBlock {
        type_: Type,
        trait_: Option<String>,
        contents: Vec<Box<Item>>,
    },
    Trait {
        name: String,
        contents: Vec<Box<Item>>,
    },
    Effect {
        name: String,
        generics: Vec<Generic>,
        contents: Vec<Operation>,
    },
    Constant {
        name: String,
        type_: Type,
        value: Expression,
    },
}

pub(crate) enum Type {
    Boolean,
    Number,
    NaturalNumber,
    NaturalNumber8Bit,
    NaturalNumber16Bit,
    NaturalNumber32Bit,
    NaturalNumber64Bit,
    IntegerNumber,
    IntegerNumber8Bit,
    IntegerNumber16Bit,
    IntegerNumber32Bit,
    IntegerNumber64Bit,
    FloatingPointNumber,
    FloatingPointNumber32Bit,
    FloatingPointNumber64Bit,
    Character,
    String,
    List(Box<Type>),
    Map { key: Box<Type>, value: Box<Type> },
    Any { trait_bounds: Vec<String> },
    AlgebraicType(String),
    Function { arguments: Vec<Type>, return_type: Box<Type>, effects: Vec<EffectType> },
    Unit,
    Never,
}

pub(crate) enum Expression {
    Boolean(bool),
    NaturalNumber8Bit(u8),
    NaturalNumber16Bit(u16),
    NaturalNumber32Bit(u32),
    NaturalNumber64Bit(u64),
    IntegerNumber8Bit(i8),
    IntegerNumber16Bit(i16),
    IntegerNumber32Bit(i32),
    IntegerNumber64Bit(i64),
}

//--> Functions & Impls <--

pub(crate) fn generate(cst: ConcreteSyntaxNode) -> Result<AbstractSyntaxTree> {
    todo!()
}