// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::field_expression::FieldExpression;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-documentation"}}}
/// A Structure Expression
///
/// This is how we create instances in dwarf.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructExpression {
    pub bug: Uuid,
    pub id: usize,
    /// R39: [`StructExpression`] '' [`WoogStruct`]
    pub woog_struct: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-implementation"}}}
impl StructExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-new"}}}
    /// Inter a new 'Struct Expression' in the store, and return it's `id`.
    pub async fn new(
        bug: Uuid,
        woog_struct: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<StructExpression>> {
        let woog_struct = woog_struct.read().await.id;
        store
            .inter_struct_expression(|id| {
                Arc::new(RwLock::new(StructExpression {
                    bug,
                    id,
                    woog_struct,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R39(1-*)
    pub async fn r39_woog_struct<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<WoogStruct>>> + '_ {
        span!("r39_woog_struct");
        stream::iter(vec![store.exhume_woog_struct(&self.woog_struct).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R26(1-M)
    pub async fn r26_field_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<FieldExpression>>> + '_ {
        span!("r26_field_expression");
        store
            .iter_field_expression()
            .await
            .filter_map(|field_expression| async {
                if field_expression.read().await.woog_struct == self.id {
                    Some(field_expression)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-impl-nav-subtype-to-supertype-expression"}}}
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
                if let ExpressionEnum::StructExpression(id) = expression.read().await.subtype {
                    Some(expression.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-implementation"}}}
impl PartialEq for StructExpression {
    fn eq(&self, other: &Self) -> bool {
        self.bug == other.bug && self.woog_struct == other.woog_struct
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
