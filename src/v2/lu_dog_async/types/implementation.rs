// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::item::ItemEnum;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Implementation {
    pub id: Uuid,
    /// R8: [`Implementation`] 'adds functions to a' [`WoogStruct`]
    pub model_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-implementation"}}}
impl Implementation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-new"}}}
    /// Inter a new 'Implementation' in the store, and return it's `id`.
    pub async fn new(
        model_type: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Implementation>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Implementation {
            id,
            model_type: model_type.read().await.id,
        }));
        store.inter_implementation(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-nav-forward-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*)
    pub async fn r8_woog_struct<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<WoogStruct>>> {
        span!("r8_woog_struct");
        vec![store.exhume_woog_struct(&self.model_type).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub async fn r9_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        use futures::stream::{self, StreamExt};
        span!("r9_function");
        stream::iter(
            store
                .iter_function()
                .await
                .collect::<Vec<Arc<RwLock<Function>>>>(),
        )
        .filter(|function: Arc<RwLock<Function>>| async move {
            function.read().await.impl_block == Some(self.id).collect().await
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub async fn r6_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        let mut result = Vec::new();
        for item in store.iter_item().await {
            if let ItemEnum::Implementation(id) = item.read().await.subtype {
                result.push(item.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
