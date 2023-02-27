//! Domain for generating code.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::woog_2-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog_2-module-definition"}}}
pub mod access;
pub mod block;
pub mod borrowed;
pub mod call;
pub mod expression;
pub mod expression_statement;
pub mod grace_type;
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
pub mod value;
pub mod variable;
pub mod visibility;
pub mod x_let;
pub mod x_macro;

pub use crate::v2::woog_2::access::Access;
pub use crate::v2::woog_2::block::Block;
pub use crate::v2::woog_2::borrowed::BORROWED;
pub use crate::v2::woog_2::call::CALL;
pub use crate::v2::woog_2::expression::Expression;
pub use crate::v2::woog_2::expression_statement::EXPRESSION_STATEMENT;
pub use crate::v2::woog_2::grace_type::GraceType;
pub use crate::v2::woog_2::item::ITEM;
pub use crate::v2::woog_2::krate::KRATE;
pub use crate::v2::woog_2::literal::LITERAL;
pub use crate::v2::woog_2::local::Local;
pub use crate::v2::woog_2::mutable::MUTABLE;
pub use crate::v2::woog_2::object_method::ObjectMethod;
pub use crate::v2::woog_2::owned::OWNED;
pub use crate::v2::woog_2::ownership::Ownership;
pub use crate::v2::woog_2::parameter::Parameter;
pub use crate::v2::woog_2::private::PRIVATE;
pub use crate::v2::woog_2::public::PUBLIC;
pub use crate::v2::woog_2::reference::Reference;
pub use crate::v2::woog_2::statement::Statement;
pub use crate::v2::woog_2::value::Value;
pub use crate::v2::woog_2::variable::Variable;
pub use crate::v2::woog_2::visibility::Visibility;
pub use crate::v2::woog_2::x_let::XLet;
pub use crate::v2::woog_2::x_macro::X_MACRO;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
