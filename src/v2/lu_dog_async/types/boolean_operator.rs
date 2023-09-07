// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::and::AND;
use crate::v2::lu_dog_async::types::binary::Binary;
use crate::v2::lu_dog_async::types::binary::BinaryEnum;
use crate::v2::lu_dog_async::types::or::OR;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-documentation"}}}
/// A Boolean Operaator
///
/// There are two — || and &&.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BooleanOperator {
    pub subtype: BooleanOperatorEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BooleanOperatorEnum {
    And(Uuid),
    Or(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-implementation"}}}
impl BooleanOperator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-struct-impl-new_and"}}}
    /// Inter a new BooleanOperator in the store, and return it's `id`.
    pub async fn new_and(store: &mut LuDogAsyncStore) -> Arc<RwLock<BooleanOperator>> {
        store
            .inter_boolean_operator(|id| {
                Arc::new(RwLock::new(BooleanOperator {
                    subtype: BooleanOperatorEnum::And(AND),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-struct-impl-new_or"}}}
    /// Inter a new BooleanOperator in the store, and return it's `id`.
    pub async fn new_or(store: &mut LuDogAsyncStore) -> Arc<RwLock<BooleanOperator>> {
        store
            .inter_boolean_operator(|id| {
                Arc::new(RwLock::new(BooleanOperator {
                    subtype: BooleanOperatorEnum::Or(OR),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-impl-nav-subtype-to-supertype-binary"}}}
    // Navigate to [`Binary`] across R48(isa)
    pub async fn r48_binary<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Binary>>> {
        span!("r48_binary");
        store
            .iter_binary()
            .await
            .filter_map(|binary| async move {
                if let BinaryEnum::BooleanOperator(id) = binary.read().await.subtype {
                    Some(binary.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-implementation"}}}
impl PartialEq for BooleanOperator {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
