// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_match-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::pattern::Pattern;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-documentation"}}}
/// Match a pattern to a scrutinee and evaluate a branch based on the results.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XMatch {
    pub id: usize,
    pub uniqueness_generator: Uuid,
    /// R91: [`XMatch`] 'deconstructs ' [`Expression`]
    pub scrutinee: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-implementation"}}}
impl XMatch {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-new"}}}
    /// Inter a new 'Match' in the store, and return it's `id`.
    pub async fn new(
        uniqueness_generator: Uuid,
        scrutinee: &Arc<RwLock<Expression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<XMatch>> {
        let scrutinee = scrutinee.read().await.id;
        store
            .inter_x_match(|id| {
                Arc::new(RwLock::new(XMatch {
                    id,
                    uniqueness_generator,
                    scrutinee,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-forward-to-scrutinee"}}}
    /// Navigate to [`Expression`] across R91(1-*)
    pub async fn r91_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        stream::iter(vec![store.exhume_expression(&self.scrutinee).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub async fn r87_pattern<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Pattern>>> {
        store
            .iter_pattern()
            .await
            .filter_map(|pattern| async {
                if pattern.read().await.x_match == self.id {
                    Some(pattern)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::XMatch(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-implementation"}}}
impl PartialEq for XMatch {
    fn eq(&self, other: &Self) -> bool {
        self.uniqueness_generator == other.uniqueness_generator && self.scrutinee == other.scrutinee
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
