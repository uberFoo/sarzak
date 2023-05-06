// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"comparison-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-use-statements"}}}
use std::sync::{Arc, RwLock};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::less_than_or_equal::LESS_THAN_OR_EQUAL;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-enum-documentation"}}}
/// Comparison Operators
///
/// Things like == and !=, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Comparison {
    LessThanOrEqual(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-implementation"}}}
impl Comparison {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-new-impl"}}}
    /// Create a new instance of Comparison::LessThanOrEqual
    pub fn new_less_than_or_equal(store: &LuDogStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&LESS_THAN_OR_EQUAL).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Comparison::LessThanOrEqual(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Operator>>> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Comparison(id) = operator.read().unwrap().subtype {
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
