// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::and::AND;
use crate::v2::lu_dog::types::binary::Binary;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-enum-documentation"}}}
/// A Boolean Operaator
///
/// There are two — || and &&.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BooleanOperator {
    And(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-implementation"}}}
impl BooleanOperator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-new-impl"}}}
    /// Create a new instance of BooleanOperator::And
    pub fn new_and(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_boolean_operator(&AND).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            BooleanOperator::And(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-impl-nav-subtype-to-supertype-binary"}}}
    // Navigate to [`Binary`] across R48(isa)
    pub fn r48_binary<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Binary>>> {
        span!("r48_binary");
        vec![store.exhume_binary(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}