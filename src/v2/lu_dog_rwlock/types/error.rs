// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"error-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-use-statements"}}}
use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
use crate::v2::lu_dog_rwlock::types::unknown_variable::UNKNOWN_VARIABLE;
use crate::v2::lu_dog_rwlock::types::value_type::ValueType;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-enum-documentation"}}}
/// A type to signify an Error condition
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Error {
    UnknownVariable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-implementation"}}}
impl Error {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-new-impl"}}}
    /// Create a new instance of Error::UnknownVariable
    pub fn new_unknown_variable(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_error(&UNKNOWN_VARIABLE).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Error::UnknownVariable(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r1_value_type");
        vec![store.exhume_value_type(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
