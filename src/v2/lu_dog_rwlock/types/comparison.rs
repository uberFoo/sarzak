// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"comparison-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-use-statements"}}}
use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
use crate::v2::lu_dog_rwlock::types::equal::EQUAL;
use crate::v2::lu_dog_rwlock::types::greater_than::GREATER_THAN;
use crate::v2::lu_dog_rwlock::types::greater_than_or_equal::GREATER_THAN_OR_EQUAL;
use crate::v2::lu_dog_rwlock::types::less_than_or_equal::LESS_THAN_OR_EQUAL;
use crate::v2::lu_dog_rwlock::types::not_equal::NOT_EQUAL;
use crate::v2::lu_dog_rwlock::types::operator::Operator;
use crate::v2::lu_dog_rwlock::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
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
    Equal(Uuid),
    GreaterThan(Uuid),
    GreaterThanOrEqual(Uuid),
    LessThanOrEqual(Uuid),
    NotEqual(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-implementation"}}}
impl Comparison {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-new-impl"}}}
    /// Create a new instance of Comparison::Equal
    pub fn new_equal(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&EQUAL).unwrap()
    }

    /// Create a new instance of Comparison::GreaterThan
    pub fn new_greater_than(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&GREATER_THAN).unwrap()
    }

    /// Create a new instance of Comparison::GreaterThanOrEqual
    pub fn new_greater_than_or_equal(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&GREATER_THAN_OR_EQUAL).unwrap()
    }

    /// Create a new instance of Comparison::LessThanOrEqual
    pub fn new_less_than_or_equal(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&LESS_THAN_OR_EQUAL).unwrap()
    }

    /// Create a new instance of Comparison::NotEqual
    pub fn new_not_equal(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&NOT_EQUAL).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Comparison::Equal(id) => *id,
            Comparison::GreaterThan(id) => *id,
            Comparison::GreaterThanOrEqual(id) => *id,
            Comparison::LessThanOrEqual(id) => *id,
            Comparison::NotEqual(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Comparison(id) = operator.read().unwrap().subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
