// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"comparison-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::equal::EQUAL;
use crate::v2::lu_dog_async::types::greater_than::GREATER_THAN;
use crate::v2::lu_dog_async::types::greater_than_or_equal::GREATER_THAN_OR_EQUAL;
use crate::v2::lu_dog_async::types::less_than_or_equal::LESS_THAN_OR_EQUAL;
use crate::v2::lu_dog_async::types::not_equal::NOT_EQUAL;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::operator::OperatorEnum;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
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
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub async fn new_equal(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&EQUAL).await.unwrap()
    }

    /// Create a new instance of Comparison::GreaterThan
    pub async fn new_greater_than(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&GREATER_THAN).await.unwrap()
    }

    /// Create a new instance of Comparison::GreaterThanOrEqual
    pub async fn new_greater_than_or_equal(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store
            .exhume_comparison(&GREATER_THAN_OR_EQUAL)
            .await
            .unwrap()
    }

    /// Create a new instance of Comparison::LessThanOrEqual
    pub async fn new_less_than_or_equal(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&LESS_THAN_OR_EQUAL).await.unwrap()
    }

    /// Create a new instance of Comparison::NotEqual
    pub async fn new_not_equal(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_comparison(&NOT_EQUAL).await.unwrap()
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
    pub async fn r47_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        let mut result = Vec::new();
        for operator in store.iter_operator().await {
            if let OperatorEnum::Comparison(id) = operator.read().await.subtype {
                result.push(operator.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
