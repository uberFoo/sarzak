// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use std::sync::{Arc, RwLock};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::addition::ADDITION;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::operator::OperatorEnum;
use crate::v2::lu_dog::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-enum-documentation"}}}
/// Binary Operators
///
/// +, -, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Binary {
    Addition(Uuid),
    Subtraction(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl Binary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-new-impl"}}}
    /// Create a new instance of Binary::Addition
    pub fn new_addition(store: &LuDogStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&ADDITION).unwrap()
    }

    /// Create a new instance of Binary::Subtraction
    pub fn new_subtraction(store: &LuDogStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&SUBTRACTION).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Binary::Addition(id) => *id,
            Binary::Subtraction(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Operator>>> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.read().unwrap().subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()] // 💥
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
