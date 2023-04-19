// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"error-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::unknown_variable::UNKNOWN_VARIABLE;
use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-enum-documentation"}}}
/// A type to signify an Error condition
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Error {
    UnknownVariable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-implementation"}}}
impl Error {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-new-impl"}}}
    /// Create a new instance of Error::UnknownVariable
    pub fn new_unknown_variable() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::UnknownVariable(UNKNOWN_VARIABLE)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-struct-impl-new_"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Error::UnknownVariable(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-impl-nav-subtype-to-supertype-expression"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
