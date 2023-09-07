// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"method_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::call::Call;
use crate::v2::lu_dog_async::types::call::CallEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-documentation"}}}
/// A Method Call
///
/// This is when you call a function on an instance of a struct. The name attribute is the name
///  of the method.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MethodCall {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-implementation"}}}
impl MethodCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-impl-new"}}}
    /// Inter a new 'Method Call' in the store, and return it's `id`.
    pub async fn new(name: String, store: &mut LuDogAsyncStore) -> Arc<RwLock<MethodCall>> {
        store
            .inter_method_call(|id| {
                Arc::new(RwLock::new(MethodCall {
                    id,
                    name: name.to_owned(),
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub async fn r30_call<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Call>>> {
        span!("r30_call");
        store
            .iter_call()
            .await
            .filter_map(|call| async move {
                if let CallEnum::MethodCall(id) = call.read().await.subtype {
                    Some(call.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-implementation"}}}
impl PartialEq for MethodCall {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
