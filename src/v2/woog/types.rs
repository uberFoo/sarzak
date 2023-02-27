//! Domain for generating code.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::woog_2-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog_2-module-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::woog-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-module-definition"}}}
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
pub mod woog_option;
pub mod x_let;
pub mod x_macro;

pub use crate::v2::woog::access::Access;
pub use crate::v2::woog::block::Block;
pub use crate::v2::woog::borrowed::BORROWED;
pub use crate::v2::woog::call::Call;
pub use crate::v2::woog::expression::Expression;
pub use crate::v2::woog::expression_statement::EXPRESSION_STATEMENT;
pub use crate::v2::woog::grace_type::GraceType;
pub use crate::v2::woog::item::ITEM;
pub use crate::v2::woog::krate::KRATE;
pub use crate::v2::woog::literal::LITERAL;
pub use crate::v2::woog::local::Local;
pub use crate::v2::woog::mutable::MUTABLE;
pub use crate::v2::woog::object_method::ObjectMethod;
pub use crate::v2::woog::owned::OWNED;
pub use crate::v2::woog::ownership::Ownership;
pub use crate::v2::woog::parameter::Parameter;
pub use crate::v2::woog::private::PRIVATE;
pub use crate::v2::woog::public::PUBLIC;
pub use crate::v2::woog::reference::Reference;
pub use crate::v2::woog::statement::Statement;
pub use crate::v2::woog::value::Value;
pub use crate::v2::woog::variable::Variable;
pub use crate::v2::woog::visibility::Visibility;
pub use crate::v2::woog::woog_option::WoogOption;
pub use crate::v2::woog::x_let::XLet;
pub use crate::v2::woog::x_macro::X_MACRO;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
