//! Domain for generating code.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::woog_single-module-definition-file"}}}
pub mod access;
pub mod block;
pub mod borrowed;
pub mod call;
pub mod constant;
pub mod enumeration;
pub mod enumeration_field;
pub mod expression;
pub mod expression_statement;
pub mod field;
pub mod function;
pub mod generation_unit;
pub mod grace_type;
pub mod implementation;
pub mod item;
pub mod krate;
pub mod literal;
pub mod local;
pub mod mutable;
pub mod object_method;
pub mod owned;
pub mod ownership;
pub mod parameter;
pub mod private;
pub mod public;
pub mod reference;
pub mod statement;
pub mod structure;
pub mod structure_field;
pub mod symbol_table;
pub mod time_stamp;
pub mod usize;
pub mod variable;
pub mod visibility;
pub mod woog_option;
pub mod x_let;
pub mod x_macro;
pub mod x_value;

pub use crate::v2::woog_single::access::Access;
pub use crate::v2::woog_single::block::Block;
pub use crate::v2::woog_single::borrowed::Borrowed;
pub use crate::v2::woog_single::borrowed::BORROWED;
pub use crate::v2::woog_single::call::Call;
pub use crate::v2::woog_single::constant::Constant;
pub use crate::v2::woog_single::enumeration::Enumeration;
pub use crate::v2::woog_single::enumeration_field::EnumerationField;
pub use crate::v2::woog_single::expression::Expression;
pub use crate::v2::woog_single::expression_statement::ExpressionStatement;
pub use crate::v2::woog_single::expression_statement::EXPRESSION_STATEMENT;
pub use crate::v2::woog_single::field::Field;
pub use crate::v2::woog_single::function::Function;
pub use crate::v2::woog_single::function::FunctionEnum;
pub use crate::v2::woog_single::generation_unit::GenerationUnit;
pub use crate::v2::woog_single::grace_type::GraceType;
pub use crate::v2::woog_single::implementation::Implementation;
pub use crate::v2::woog_single::implementation::IMPLEMENTATION;
pub use crate::v2::woog_single::item::Item;
pub use crate::v2::woog_single::krate::Krate;
pub use crate::v2::woog_single::krate::KRATE;
pub use crate::v2::woog_single::literal::Literal;
pub use crate::v2::woog_single::literal::LITERAL;
pub use crate::v2::woog_single::local::Local;
pub use crate::v2::woog_single::mutable::Mutable;
pub use crate::v2::woog_single::mutable::MUTABLE;
pub use crate::v2::woog_single::object_method::ObjectMethod;
pub use crate::v2::woog_single::owned::Owned;
pub use crate::v2::woog_single::owned::OWNED;
pub use crate::v2::woog_single::ownership::Ownership;
pub use crate::v2::woog_single::parameter::Parameter;
pub use crate::v2::woog_single::private::Private;
pub use crate::v2::woog_single::private::PRIVATE;
pub use crate::v2::woog_single::public::Public;
pub use crate::v2::woog_single::public::PUBLIC;
pub use crate::v2::woog_single::reference::Reference;
pub use crate::v2::woog_single::statement::Statement;
pub use crate::v2::woog_single::statement::StatementEnum;
pub use crate::v2::woog_single::structure::Structure;
pub use crate::v2::woog_single::structure_field::StructureField;
pub use crate::v2::woog_single::symbol_table::SymbolTable;
pub use crate::v2::woog_single::time_stamp::TimeStamp;
pub use crate::v2::woog_single::usize::Usize;
pub use crate::v2::woog_single::usize::USIZE;
pub use crate::v2::woog_single::variable::Variable;
pub use crate::v2::woog_single::variable::VariableEnum;
pub use crate::v2::woog_single::visibility::Visibility;
pub use crate::v2::woog_single::woog_option::WoogOption;
pub use crate::v2::woog_single::x_let::XLet;
pub use crate::v2::woog_single::x_macro::XMacro;
pub use crate::v2::woog_single::x_macro::X_MACRO;
pub use crate::v2::woog_single::x_value::XValue;
pub use crate::v2::woog_single::x_value::XValueEnum;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
