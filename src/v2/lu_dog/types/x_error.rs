// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_error-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::unknown_variable::UNKNOWN_VARIABLE;
use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-enum-documentation"}}}
/// A type to signify an Error condition
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum XError {
    UnknownVariable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-implementation"}}}
impl XError {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-new-impl"}}}
    /// Create a new instance of XError::UnknownVariable
    pub fn new_unknown_variable(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_x_error(&UNKNOWN_VARIABLE).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::UnknownVariable(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_error-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store.exhume_value_type(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
