// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_if-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::block::Block;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-documentation"}}}
/// The if Expression
///
/// It does include an else, at no extra charge!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XIf {
    pub id: usize,
    /// R52: [`XIf`] 'false block' [`Block`]
    pub false_block: Option<usize>,
    /// R44: [`XIf`] 'branches based on' [`Expression`]
    pub test: usize,
    /// R46: [`XIf`] 'when true, evaluates' [`Block`]
    pub true_block: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-implementation"}}}
impl XIf {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-new"}}}
    /// Inter a new 'If' in the store, and return it's `id`.
    pub async fn new(
        false_block: Option<&Arc<RwLock<Block>>>,
        test: &Arc<RwLock<Expression>>,
        true_block: &Arc<RwLock<Block>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<XIf>> {
        let true_block = true_block.read().await.id;
        let test = test.read().await.id;
        let block = match false_block {
            Some(block) => Some(block.read().await.id),
            None => None,
        };
        store
            .inter_x_if(|id| {
                Arc::new(RwLock::new(XIf {
                    id,
                    false_block: block,
                    test,
                    true_block,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-cond-to-false_block"}}}
    /// Navigate to [`Block`] across R52(1-*c)
    pub async fn r52_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r52_block");
        match self.false_block {
            Some(ref false_block) => vec![store.exhume_block(false_block).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-test"}}}
    /// Navigate to [`Expression`] across R44(1-*)
    pub async fn r44_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r44_expression");
        vec![store.exhume_expression(&self.test).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-true_block"}}}
    /// Navigate to [`Block`] across R46(1-*)
    pub async fn r46_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r46_block");
        vec![store.exhume_block(&self.true_block).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-impl-nav-subtype-to-supertype-expression"}}}
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
                if let ExpressionEnum::XIf(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-implementation"}}}
impl PartialEq for XIf {
    fn eq(&self, other: &Self) -> bool {
        self.false_block == other.false_block
            && self.test == other.test
            && self.true_block == other.true_block
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
