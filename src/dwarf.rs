use std::{fmt, ops};

use ansi_term::Colour;
use snafu::prelude::*;

pub mod parser;
pub mod parser_orig;
pub mod walker;

pub use parser::parse;
pub use walker::populate_lu_dog;

pub type Result<T, E = DwarfError> = std::result::Result<T, E>;

pub type Span = ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DwarfError {
    #[snafu(display("\n{}: {description}\n  --> {}..{}", Colour::Red.bold().paint("error"), span.start, span.end))]
    Parse { description: String, span: Span },
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Bool(bool),
    Float(String),
    Fn,
    Ident(String),
    Impl,
    Import,
    Integer(String),
    Let,
    // This is a type that starts with a capital letter. It's special. 💩
    Object(String),
    Op(String),
    Option,
    Print,
    Punct(char),
    Self_,
    String(String),
    Struct,
    Type(Type),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(bool_) => write!(f, "{}", bool_),
            Self::Float(num) => write!(f, "{}", num),
            Self::Fn => write!(f, "fn"),
            Self::Ident(ident) => write!(f, "{}", ident),
            Self::Impl => write!(f, "impl"),
            Self::Import => write!(f, "import"),
            Self::Integer(num) => write!(f, "{}", num),
            Self::Let => write!(f, "let"),
            Self::Object(object) => write!(f, "{}", object),
            Self::Op(op) => write!(f, "{}", op),
            Self::Option => write!(f, "Option"),
            Self::Print => write!(f, "print"),
            Self::Punct(punct) => write!(f, "{}", punct),
            Self::Self_ => write!(f, "Self"),
            Self::String(str_) => write!(f, "{}", str_),
            Self::Struct => write!(f, "struct"),
            Self::Type(type_) => write!(f, "{}", type_),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Boolean,
    Float,
    Integer,
    Option(Box<Self>),
    Self_(Box<Token>),
    String,
    UserType(Box<Token>),
    Uuid,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "bool"),
            Self::Float => write!(f, "float"),
            Self::Integer => write!(f, "int"),
            Self::Option(type_) => write!(f, "Option<{}>", type_),
            Self::Self_(type_) => write!(f, "{}", type_),
            Self::String => write!(f, "string"),
            Self::UserType(type_) => write!(f, "{}", type_),
            Self::Uuid => write!(f, "Uuid"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Spanned<Expression>),
    Item(Item),
    Let(Spanned<String>, Spanned<Expression>),
    Result(Spanned<Expression>),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Block(Vec<Spanned<Statement>>),
    BooleanLiteral(bool),
    Error,
    FieldAccess(Box<Spanned<Self>>, Spanned<String>),
    FloatLiteral(f64),
    // The first element is the function being called, the second is the list of
    // arguments.
    FunctionCall(Box<Spanned<Self>>, Vec<Spanned<Self>>),
    IntegerLiteral(i64),
    LocalVariable(String),
    MethodCall(Box<Spanned<Self>>, Spanned<String>, Vec<Spanned<Self>>),
    Print(Box<Spanned<Self>>),
    /// Static Method Call
    ///
    /// E.g., `Foo::bar()`.
    ///
    ///
    StaticMethodCall(Spanned<Token>, Spanned<String>, Vec<Spanned<Self>>),
    /// String Literal
    ///
    StringLiteral(String),
    /// Structure Expression
    ///
    /// Struct Name, Vec<Field Name, Field Value>
    Struct(Spanned<Token>, Vec<(Spanned<String>, Spanned<Self>)>),
}

/// ItemKind
///
/// The sole purpose of this is to have a unique key in the hashmap.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ItemKind {
    Function,
    Implementation,
    Import,
    Struct,
}

#[derive(Clone, Debug)]
pub enum Item {
    /// A Function Definition
    ///
    /// (Vec<(Parameter Name, Parameter Type)>, Return Type, Vec<Statement>)
    Function(
        Vec<(Spanned<String>, Spanned<Type>)>,
        Spanned<Type>,
        Vec<Spanned<Statement>>,
    ),
    /// Vec<(Function Name, Function)>
    Implementation(Vec<(Spanned<String>, Box<Item>)>),
    Import(Spanned<String>),
    /// Vec<(Field Name, Field Type)>
    Struct(Vec<(Spanned<String>, Spanned<Type>)>),
}
