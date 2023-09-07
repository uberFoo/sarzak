// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"plain-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::enum_field::EnumField;
use crate::v2::lu_dog_async::types::enum_field::EnumFieldEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-struct-documentation"}}}
/// Just a marker, no other value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plain {
    pub id: usize,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-implementation"}}}
impl Plain {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-struct-impl-new"}}}
    /// Inter a new 'Plain' in the store, and return it's `id`.
    pub async fn new(x_value: i64, store: &mut LuDogAsyncStore) -> Arc<RwLock<Plain>> {
        store
            .inter_plain(|id| Arc::new(RwLock::new(Plain { id, x_value })))
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub async fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        span!("r85_enum_field");
        store
            .iter_enum_field()
            .await
            .filter_map(|enum_field| async move {
                if let EnumFieldEnum::Plain(id) = enum_field.read().await.subtype {
                    Some(enum_field.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-implementation"}}}
impl PartialEq for Plain {
    fn eq(&self, other: &Self) -> bool {
        self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
