//! Domain for generating code.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_v2-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_v2-module-definition"}}}
pub mod borrowed;
pub mod krate;
pub mod mutability;
pub mod mutable;
pub mod object_method;
pub mod parameter;
pub mod private;
pub mod public;
pub mod visibility;

pub use crate::woog_v2::borrowed::BORROWED;
pub use crate::woog_v2::krate::KRATE;
pub use crate::woog_v2::mutability::Mutability;
pub use crate::woog_v2::mutable::MUTABLE;
pub use crate::woog_v2::object_method::ObjectMethod;
pub use crate::woog_v2::parameter::Parameter;
pub use crate::woog_v2::private::PRIVATE;
pub use crate::woog_v2::public::PUBLIC;
pub use crate::woog_v2::visibility::Visibility;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
