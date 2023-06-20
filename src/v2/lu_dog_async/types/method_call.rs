// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"method_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MethodCall {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-implementation"}}}
impl MethodCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-impl-new"}}}
    /// Inter a new 'Method Call' in the store, and return it's `id`.
    pub async fn new(name: String, store: &mut LuDogAsyncStore) -> Arc<RwLock<MethodCall>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(MethodCall { id, name }));
        store.inter_method_call(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub async fn r30_call<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Call>>> {
        span!("r30_call");
        let mut result = Vec::new();
        for call in store.iter_call().await {
            if let CallEnum::MethodCall(id) = call.read().await.subtype {
                result.push(call.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
