// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-documentation"}}}
/// A List
///
/// This is like an array, I guess. It's also like a `Vec<T>`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct List {
    pub id: usize,
    /// R36: [`List`] '' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-implementation"}}}
impl List {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-impl-new"}}}
    /// Inter a new 'List' in the store, and return it's `id`.
    pub async fn new(
        ty: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<List>> {
        let ty = ty.read().await.id;
        store
            .inter_list(|id| Arc::new(RwLock::new(List { id, ty })))
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R36(1-*)
    pub async fn r36_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ValueType>>> + '_ {
        span!("r36_value_type");
        stream::iter(vec![store.exhume_value_type(&self.ty).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub async fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r1_value_type");
        store
            .iter_value_type()
            .await
            .filter_map(|value_type| async move {
                if let ValueTypeEnum::List(id) = value_type.read().await.subtype {
                    Some(value_type.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-implementation"}}}
impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
