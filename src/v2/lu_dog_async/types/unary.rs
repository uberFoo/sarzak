// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::negation::NEGATION;
use crate::v2::lu_dog_async::types::not::NOT;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::operator::OperatorEnum;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-documentation"}}}
/// Unary Operators
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Unary {
    Negation(Uuid),
    Not(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-implementation"}}}
impl Unary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-new-impl"}}}
    /// Create a new instance of Unary::Negation
    pub async fn new_negation(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_unary(&NEGATION).await.unwrap()
    }

    /// Create a new instance of Unary::Not
    pub async fn new_not(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_unary(&NOT).await.unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Unary::Negation(id) => *id,
            Unary::Not(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub async fn r47_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        let mut result = Vec::new();
        for operator in store.iter_operator().await {
            if let OperatorEnum::Unary(id) = operator.read().await.subtype {
                result.push(operator.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
