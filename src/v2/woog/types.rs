//! Domain for generating code.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::woog-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-module-definition"}}}
pub mod borrowed;
pub mod krate;
pub mod mutability;
pub mod mutable;
pub mod object_method;
pub mod parameter;
pub mod private;
pub mod public;
pub mod visibility;

pub use crate::v2::woog::borrowed::BORROWED;
pub use crate::v2::woog::krate::KRATE;
pub use crate::v2::woog::mutability::Mutability;
pub use crate::v2::woog::mutable::MUTABLE;
pub use crate::v2::woog::object_method::ObjectMethod;
pub use crate::v2::woog::parameter::Parameter;
pub use crate::v2::woog::private::PRIVATE;
pub use crate::v2::woog::public::PUBLIC;
pub use crate::v2::woog::visibility::Visibility;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
