//! A blank domain
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::lu_dog-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-module-definition"}}}
pub mod field;
pub mod function;
pub mod implementation;
pub mod item;
pub mod model_type;
pub mod none;
pub mod some;
pub mod value_type;
pub mod woog_option;

pub use crate::v2::lu_dog::field::Field;
pub use crate::v2::lu_dog::function::Function;
pub use crate::v2::lu_dog::implementation::Implementation;
pub use crate::v2::lu_dog::item::Item;
pub use crate::v2::lu_dog::model_type::ModelType;
pub use crate::v2::lu_dog::none::NONE;
pub use crate::v2::lu_dog::some::Some;
pub use crate::v2::lu_dog::value_type::ValueType;
pub use crate::v2::lu_dog::woog_option::WoogOption;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
