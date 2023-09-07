// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"external_implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::body::Body;
use crate::v2::lu_dog_async::types::body::BodyEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-documentation"}}}
/// Some extern source of the function’s body.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalImplementation {
    pub function: String,
    pub id: usize,
    pub x_model: String,
    pub object: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-implementation"}}}
impl ExternalImplementation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-impl-new"}}}
    /// Inter a new 'External Implementation' in the store, and return it's `id`.
    pub async fn new(
        function: String,
        x_model: String,
        object: String,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ExternalImplementation>> {
        store
            .inter_external_implementation(|id| {
                Arc::new(RwLock::new(ExternalImplementation {
                    function: function.to_owned(),
                    id,
                    x_model: x_model.to_owned(),
                    object: object.to_owned(),
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub async fn r80_body<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Body>>> {
        span!("r80_body");
        store
            .iter_body()
            .await
            .filter_map(|body| async move {
                if let BodyEnum::ExternalImplementation(id) = body.read().await.subtype {
                    Some(body.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-implementation"}}}
impl PartialEq for ExternalImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.function == other.function
            && self.x_model == other.x_model
            && self.object == other.object
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
