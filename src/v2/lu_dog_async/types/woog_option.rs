// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_option-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use crate::v2::lu_dog_async::types::z_none::Z_NONE;
use crate::v2::lu_dog_async::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-documentation"}}}
/// An Optional Type
///
/// This type is either `None` or `Some(`[Type]`)`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WoogOption {
    pub subtype: WoogOptionEnum,
    pub id: usize,
    /// R2: [`WoogOption`] 'has a' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum WoogOptionEnum {
    ZNone(Uuid),
    ZSome(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-implementation"}}}
impl WoogOption {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_none"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub async fn new_z_none(
        ty: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<WoogOption>> {
        let ty = ty.read().await.id;
        store
            .inter_woog_option(|id| {
                Arc::new(RwLock::new(WoogOption {
                    ty, // (b)
                    subtype: WoogOptionEnum::ZNone(Z_NONE),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_some"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub async fn new_z_some(
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<ZSome>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<WoogOption>> {
        let s_id = subtype.read().await.id;
        let ty = ty.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_woog_option(|id| {
                Arc::new(RwLock::new(WoogOption {
                    ty, // (b)
                    subtype: WoogOptionEnum::ZSome(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R2(1-*)
    pub async fn r2_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ValueType>>> + '_ {
        span!("r2_value_type");
        stream::iter(vec![store.exhume_value_type(&self.ty).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-impl-nav-subtype-to-supertype-value_type"}}}
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
                if let ValueTypeEnum::WoogOption(id) = value_type.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-implementation"}}}
impl PartialEq for WoogOption {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
