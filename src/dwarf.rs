use std::{fmt, ops, path::PathBuf};

use ansi_term::Colour;
use clap::Args;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    lu_dog::{
        store::ObjectStore as LuDogStore,
        types::{Function, ValueType, WoogOption},
        List, Reference,
    },
    sarzak::{store::ObjectStore as SarzakStore, types::Ty},
};

pub mod parser;
pub mod parser_orig;
pub mod walker;

pub use parser::{parse_dwarf, parse_line};
pub use walker::{inter_statement, populate_lu_dog};

pub type Result<T, E = DwarfError> = std::result::Result<T, E>;

pub type Span = ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DwarfError {
    #[snafu(display("\n{}: {description}\n  --> {}..{}", Colour::Red.bold().paint("error"), span.start, span.end))]
    Parse { description: String, span: Span },
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct DwarfOptions {
    /// Dwarf Source File
    ///
    /// Path to the source file to compile.
    source: PathBuf,
    /// Model File
    ///
    /// Path to the model, corresponding to the source file, to build the
    /// Lu-Dog domain.
    model: PathBuf,
    /// Meta-Model File
    ///
    /// Path to the meta-model, sarzak.
    sarzak: PathBuf,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
    As,
    Bool(bool),
    Float(String),
    Fn,
    // Global,
    Ident(String),
    Impl,
    Import,
    Integer(String),
    Let,
    None,
    Op(String),
    Option,
    Print,
    Punct(char),
    Self_,
    SmallSelf,
    Some,
    String(String),
    Struct,
    Type(Type),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::As => write!(f, "as"),
            Self::Bool(bool_) => write!(f, "{}", bool_),
            Self::Float(num) => write!(f, "{}", num),
            Self::Fn => write!(f, "fn"),
            // Self::Global => write!(f, "global"),
            Self::Ident(ident) => write!(f, "{}", ident),
            Self::Impl => write!(f, "impl"),
            Self::Import => write!(f, "import"),
            Self::Integer(num) => write!(f, "{}", num),
            Self::Let => write!(f, "let"),
            Self::None => write!(f, "None"),
            Self::Op(op) => write!(f, "{}", op),
            Self::Option => write!(f, "Option"),
            Self::Print => write!(f, "print"),
            Self::Punct(punct) => write!(f, "{}", punct),
            Self::Self_ => write!(f, "Self"),
            Self::SmallSelf => write!(f, "self"),
            Self::Some => write!(f, "Some"),
            Self::String(str_) => write!(f, "{}", str_),
            Self::Struct => write!(f, "struct"),
            Self::Type(type_) => write!(f, "{}", type_),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Boolean,
    Empty,
    Float,
    Integer,
    List(Box<Self>),
    Option(Box<Self>),
    Reference(Box<Self>),
    Self_(Box<Token>),
    String,
    UserType(Box<Token>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "bool"),
            Self::Empty => write!(f, "()"),
            Self::Float => write!(f, "float"),
            Self::Integer => write!(f, "int"),
            Self::List(type_) => write!(f, "[{}]", type_),
            Self::Option(type_) => write!(f, "Option<{}>", type_),
            Self::Reference(type_) => write!(f, "&{}", type_),
            Self::Self_(type_) => write!(f, "{}", type_),
            Self::String => write!(f, "string"),
            Self::UserType(type_) => write!(f, "{}", type_),
        }
    }
}

impl Type {
    pub fn into_value_type(
        &self,
        store: &mut LuDogStore,
        model: &SarzakStore,
        sarzak: &SarzakStore,
    ) -> ValueType {
        match self {
            Type::Boolean => {
                let ty = Ty::new_boolean();
                ValueType::new_ty(&ty, store)
            }
            Type::Empty => ValueType::new_empty(),
            Type::Float => {
                let ty = Ty::new_float();
                ValueType::new_ty(&ty, store)
            }
            Type::Integer => {
                let ty = Ty::new_integer();
                ValueType::new_ty(&ty, store)
            }
            Type::List(type_) => {
                let ty = type_.into_value_type(store, model, sarzak);
                let list = List::new(&ty, store);
                ValueType::new_list(&list, store)
            }
            Type::Option(type_) => {
                let ty = type_.into_value_type(store, model, sarzak);
                let option = WoogOption::new_z_none(&ty, store);
                ValueType::new_woog_option(&option, store)
            }
            Type::Reference(type_) => {
                let ty = type_.into_value_type(store, model, sarzak);
                let reference = Reference::new(Uuid::new_v4(), false, &ty, store);
                ValueType::new_reference(&reference, store)
            }
            Type::Self_(type_) => panic!("Self is deprecated."),
            Type::String => {
                let ty = Ty::new_s_string();
                ValueType::new_ty(&ty, store)
            }
            Type::UserType(type_) => {
                let name = if let Token::Ident(name) = &**type_ {
                    name
                } else {
                    panic!("Expected UserType to be Token::Object.")
                };

                let ty = if let Some(obj_id) = model.exhume_object_id_by_name(&name) {
                    model.exhume_ty(obj_id).unwrap()
                } else {
                    let obj_id = sarzak.exhume_object_id_by_name(&name).unwrap();
                    sarzak.exhume_ty(obj_id).unwrap()
                };

                ValueType::new_ty(&ty, store)
            }
        }
    }
}

// impl From<(&Type, &mut LuDogStore, &SarzakStore)> for ValueType {
//     fn from((type_, store, model): (&Type, &mut LuDogStore, &SarzakStore)) -> Self {
//         match type_ {
//             Type::Boolean => {
//                 let ty = Ty::new_boolean();
//                 ValueType::new_ty(&ty, store)
//             }
//             Type::Empty => ValueType::new_empty(),
//             Type::Float => {
//                 let ty = Ty::new_float();
//                 ValueType::new_ty(&ty, store)
//             }
//             Type::Integer => {
//                 let ty = Ty::new_integer();
//                 ValueType::new_ty(&ty, store)
//             }
//             Type::Option(type_) => {
//                 let ty = (&**type_, &store, &model).into();
//                 let option = WoogOption::new_none(&ty, store);
//                 ValueType::new_woog_option(&option, store)
//             }
//             Type::Self_(type_) => panic!("Self is deprecated."),
//             Type::String => {
//                 let ty = Ty::new_s_string();
//                 ValueType::new_ty(&ty, store)
//             }
//             Type::UserType(type_) => {
//                 let name = if let Token::Object(name) = &**type_ {
//                     name
//                 } else {
//                     panic!("Expected UserType to be Token::Object.")
//                 };
//                 let obj_id = model.exhume_object_id_by_name(&name).unwrap();
//                 let ty = model.exhume_ty(obj_id).unwrap();
//                 ValueType::new_ty(&ty, store)
//             }
//             Type::Uuid => {
//                 let ty = Ty::new_s_uuid();
//                 ValueType::new_ty(&ty, store)
//             }
//         }
//     }
// }

#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Spanned<Expression>),
    Item(Item),
    Let(Spanned<String>, Option<Spanned<Type>>, Spanned<Expression>),
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
    List(Vec<Spanned<Self>>),
    LocalVariable(String),
    MethodCall(Box<Spanned<Self>>, Spanned<String>, Vec<Spanned<Self>>),
    None,
    Print(Box<Spanned<Self>>),
    Some(Box<Spanned<Self>>),
    /// Static Method Call
    ///
    /// E.g., `Foo::bar()`.
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
    Import(Spanned<String>, Option<Spanned<String>>),
    /// Vec<(Field Name, Field Type)>
    Struct(Vec<(Spanned<String>, Spanned<Type>)>),
}
