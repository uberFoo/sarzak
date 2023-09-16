// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-documentation"}}}
/// A Field in a data structure
///
/// A field has a name, and a type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Field {
    pub id: usize,
    pub name: String,
    /// R7: [`Field`] 'comprises a' [`WoogStruct`]
    pub x_model: usize,
    /// R5: [`Field`] 'has a' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-implementation"}}}
impl Field {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-new"}}}
    /// Inter a new 'Field' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        x_model: &Arc<RwLock<WoogStruct>>,
        ty: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Field>> {
        let x_model = x_model.read().await.id;
        let ty = ty.read().await.id;
        store
            .inter_field(|id| {
                Arc::new(RwLock::new(Field {
                    id,
                    name: name.to_owned(),
                    x_model,
                    ty,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-forward-to-x_model"}}}
    /// Navigate to [`WoogStruct`] across R7(1-*)
    pub async fn r7_woog_struct<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<WoogStruct>>> + '_ {
        span!("r7_woog_struct");
        stream::iter(vec![store.exhume_woog_struct(&self.x_model).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R5(1-*)
    pub async fn r5_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ValueType>>> + '_ {
        span!("r5_value_type");
        stream::iter(vec![store.exhume_value_type(&self.ty).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub async fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        span!("r67_field_access_target");
        store
            .iter_field_access_target()
            .await
            .filter_map(|field_access_target| async move {
                if let FieldAccessTargetEnum::Field(id) = field_access_target.read().await.subtype {
                    Some(field_access_target.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-implementation"}}}
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.x_model == other.x_model && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
