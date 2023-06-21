// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::and::AND;
use crate::v2::lu_dog_async::types::binary::Binary;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
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
    pub async fn new_and(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_boolean_operator(&AND).await.unwrap()
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
    pub async fn r48_binary<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Binary>>> {
        span!("r48_binary");
        vec![store.exhume_binary(&self.id()).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
