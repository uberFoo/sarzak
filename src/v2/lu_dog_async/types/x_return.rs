// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_return-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-documentation"}}}
/// The Return Expression
///
/// It’s an expression, and not a statement. Isn’t that interesting?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XReturn {
    pub id: usize,
    /// R45: [`XReturn`] 'evaluates and returns' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-implementation"}}}
impl XReturn {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-impl-new"}}}
    /// Inter a new 'Return' in the store, and return it's `id`.
    pub async fn new(
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<XReturn>> {
        let expression = expression.read().await.id;
        store
            .inter_x_return(|id| Arc::new(RwLock::new(XReturn { id, expression })))
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R45(1-*)
    pub async fn r45_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        span!("r45_expression");
        stream::iter(vec![store.exhume_expression(&self.expression).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-impl-nav-subtype-to-supertype-expression"}}}
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
                if let ExpressionEnum::XReturn(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-implementation"}}}
impl PartialEq for XReturn {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
