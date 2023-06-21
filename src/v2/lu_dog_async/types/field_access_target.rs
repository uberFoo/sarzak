// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::field::Field;
use crate::v2::lu_dog_async::types::field_access::FieldAccess;
use crate::v2::lu_dog_async::types::function::Function;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FieldAccessTarget {
    Field(Uuid),
    Function(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-new-impl"}}}
    /// Create a new instance of FieldAccessTarget::Field
    pub async fn new_field(
        field: &Arc<RwLock<Field>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = field.read().await.id;
        if let Some(field) = store.exhume_field_access_target(&id).await {
            field
        } else {
            let new = Arc::new(RwLock::new(Self::Field(id)));
            store.inter_field_access_target(new.clone()).await;
            new
        }
    }

    /// Create a new instance of FieldAccessTarget::Function
    pub async fn new_function(
        function: &Arc<RwLock<Function>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = function.read().await.id;
        if let Some(function) = store.exhume_field_access_target(&id).await {
            function
        } else {
            let new = Arc::new(RwLock::new(Self::Function(id)));
            store.inter_field_access_target(new.clone()).await;
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            FieldAccessTarget::Field(id) => *id,
            FieldAccessTarget::Function(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub async fn r65_field_access<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldAccess>>> {
        span!("r65_field_access");
        let mut result = Vec::new();
        for field_access in store.iter_field_access().await {
            if field_access.read().await.field == self.id() {
                result.push(field_access)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
