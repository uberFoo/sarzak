// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::block::Block;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::span::Span;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::variable::Variable;
use crate::v2::lu_dog_async::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-documentation"}}}
/// A Value
///
/// A value has a Type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XValue {
    pub subtype: XValueEnum,
    pub id: Uuid,
    /// R33: [`XValue`] '' [`Block`]
    pub block: Uuid,
    /// R24: [`XValue`] 'is decoded by a' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum XValueEnum {
    Expression(Uuid),
    Variable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl XValue {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_expression"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub async fn new_expression(
        block: &Arc<RwLock<Block>>,
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<Expression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<XValue>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XValue {
            block: block.read().await.id,
            ty: ty.read().await.id(),
            subtype: XValueEnum::Expression(subtype.read().await.id()),
            id,
        }));
        store.inter_x_value(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_variable"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub async fn new_variable(
        block: &Arc<RwLock<Block>>,
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<Variable>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<XValue>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XValue {
            block: block.read().await.id,
            ty: ty.read().await.id(),
            subtype: XValueEnum::Variable(subtype.read().await.id),
            id,
        }));
        store.inter_x_value(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R33(1-*)
    pub async fn r33_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r33_block");
        vec![store.exhume_block(&self.block).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R24(1-*)
    pub async fn r24_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r24_value_type");
        vec![store.exhume_value_type(&self.ty).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_M-to-z_some"}}}
    /// Navigate to [`ZSome`] across R23(1-M)
    pub async fn r23_z_some<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<ZSome>>> {
        span!("r23_z_some");
        let mut result = Vec::new();
        for z_some in store.iter_z_some().await {
            if z_some.read().await.inner == self.id {
                result.push(z_some)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R63(1-Mc)
    pub async fn r63_span<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Span>>> {
        use futures::stream::{self, StreamExt};
        span!("r63_span");
        stream::iter(store.iter_span().await.collect::<Vec<Arc<RwLock<Span>>>>()).filter(
            |span: Arc<RwLock<Span>>| async move {
                span.read().await.x_value == Some(self.id).collect().await
            },
        )
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
