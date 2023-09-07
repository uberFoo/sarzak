// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_some-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::woog_option::WoogOption;
use crate::v2::lu_dog_async::types::woog_option::WoogOptionEnum;
use crate::v2::lu_dog_async::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-documentation"}}}
/// Some Type
///
/// This type wraps another. It's used by the supertype, `[Option]`, to represent a type that
///  may or may not exist.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZSome {
    pub id: usize,
    /// R23: [`ZSome`] 'contains' [`XValue`]
    pub inner: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-implementation"}}}
impl ZSome {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-new"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub async fn new(
        inner: &Arc<RwLock<XValue>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ZSome>> {
        let inner = inner.read().await.id;
        store
            .inter_z_some(|id| Arc::new(RwLock::new(ZSome { id, inner })))
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-nav-forward-to-inner"}}}
    /// Navigate to [`XValue`] across R23(1-*)
    pub async fn r23_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r23_x_value");
        vec![store.exhume_x_value(&self.inner).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::ZSome(id) = expression.read().await.subtype {
                    Some(expression.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-woog_option"}}}
    // Navigate to [`WoogOption`] across R3(isa)
    pub async fn r3_woog_option<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<WoogOption>>> {
        span!("r3_woog_option");
        store
            .iter_woog_option()
            .await
            .filter_map(|woog_option| async move {
                if let WoogOptionEnum::ZSome(id) = woog_option.read().await.subtype {
                    Some(woog_option.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-implementation"}}}
impl PartialEq for ZSome {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
