// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"for_loop-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-documentation"}}}
/// A For Loop Expression
///
/// An expression that matches for IDENT in EXPRESSION BLOCK.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForLoop {
    pub id: usize,
    pub ident: String,
    /// R43: [`ForLoop`] 'executes a' [`Expression`]
    pub block: usize,
    /// R42: [`ForLoop`] 'iterates over an' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-implementation"}}}
impl ForLoop {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-new"}}}
    /// Inter a new 'For Loop' in the store, and return it's `id`.
    pub async fn new(
        ident: String,
        block: &Arc<RwLock<Expression>>,
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ForLoop>> {
        let block = block.read().await.id;
        let expression = expression.read().await.id;
        store
            .inter_for_loop(|id| {
                Arc::new(RwLock::new(ForLoop {
                    id,
                    ident: ident.to_owned(),
                    block,
                    expression,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Expression`] across R43(1-*)
    pub async fn r43_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        stream::iter(vec![store.exhume_expression(&self.block).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R42(1-*)
    pub async fn r42_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        stream::iter(vec![store.exhume_expression(&self.expression).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::ForLoop(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-implementation"}}}
impl PartialEq for ForLoop {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
            && self.block == other.block
            && self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
