// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"comparison-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::equal::EQUAL;
use crate::v2::lu_dog_async::types::greater_than::GREATER_THAN;
use crate::v2::lu_dog_async::types::greater_than_or_equal::GREATER_THAN_OR_EQUAL;
use crate::v2::lu_dog_async::types::less_than::LESS_THAN;
use crate::v2::lu_dog_async::types::less_than_or_equal::LESS_THAN_OR_EQUAL;
use crate::v2::lu_dog_async::types::not_equal::NOT_EQUAL;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-documentation"}}}
/// Comparison Operators
///
/// Things like == and !=, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comparison {
    pub subtype: ComparisonEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ComparisonEnum {
    Equal(Uuid),
    GreaterThan(Uuid),
    GreaterThanOrEqual(Uuid),
    LessThan(Uuid),
    LessThanOrEqual(Uuid),
    NotEqual(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-implementation"}}}
impl Comparison {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub async fn new_equal(store: &mut LuDogAsyncStore) -> Arc<RwLock<Comparison>> {
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: ComparisonEnum::Equal(EQUAL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_greater_than"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub async fn new_greater_than(store: &mut LuDogAsyncStore) -> Arc<RwLock<Comparison>> {
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: ComparisonEnum::GreaterThan(GREATER_THAN),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_greater_than_or_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub async fn new_greater_than_or_equal(store: &mut LuDogAsyncStore) -> Arc<RwLock<Comparison>> {
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: ComparisonEnum::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_less_than"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub async fn new_less_than(store: &mut LuDogAsyncStore) -> Arc<RwLock<Comparison>> {
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: ComparisonEnum::LessThan(LESS_THAN),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_less_than_or_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub async fn new_less_than_or_equal(store: &mut LuDogAsyncStore) -> Arc<RwLock<Comparison>> {
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: ComparisonEnum::LessThanOrEqual(LESS_THAN_OR_EQUAL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_not_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub async fn new_not_equal(store: &mut LuDogAsyncStore) -> Arc<RwLock<Comparison>> {
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: ComparisonEnum::NotEqual(NOT_EQUAL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub async fn r47_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        store
            .iter_operator()
            .await
            .filter_map(|operator| async move {
                if let OperatorEnum::Comparison(id) = operator.read().await.subtype {
                    Some(operator.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-implementation"}}}
impl PartialEq for Comparison {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
