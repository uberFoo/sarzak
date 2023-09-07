// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"range_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::from::FROM;
use crate::v2::lu_dog_async::types::full::FULL;
use crate::v2::lu_dog_async::types::inclusive::INCLUSIVE;
use crate::v2::lu_dog_async::types::to::TO;
use crate::v2::lu_dog_async::types::to_inclusive::TO_INCLUSIVE;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RangeExpression {
    pub subtype: RangeExpressionEnum,
    pub id: usize,
    /// R58: [`RangeExpression`] '' [`Expression`]
    pub lhs: Option<usize>,
    /// R59: [`RangeExpression`] '' [`Expression`]
    pub rhs: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum RangeExpressionEnum {
    From(Uuid),
    Full(Uuid),
    Inclusive(Uuid),
    To(Uuid),
    ToInclusive(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-implementation"}}}
impl RangeExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_from"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_from(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let lhs = match lhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_range_expression(|id| {
                Arc::new(RwLock::new(RangeExpression {
                    lhs,
                    rhs,
                    subtype: RangeExpressionEnum::From(FROM),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_full"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_full(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let lhs = match lhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_range_expression(|id| {
                Arc::new(RwLock::new(RangeExpression {
                    lhs,
                    rhs,
                    subtype: RangeExpressionEnum::Full(FULL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let lhs = match lhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_range_expression(|id| {
                Arc::new(RwLock::new(RangeExpression {
                    lhs,
                    rhs,
                    subtype: RangeExpressionEnum::Inclusive(INCLUSIVE),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_to(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let lhs = match lhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_range_expression(|id| {
                Arc::new(RwLock::new(RangeExpression {
                    lhs,
                    rhs,
                    subtype: RangeExpressionEnum::To(TO),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_to_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let lhs = match lhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_range_expression(|id| {
                Arc::new(RwLock::new(RangeExpression {
                    lhs,
                    rhs,
                    subtype: RangeExpressionEnum::ToInclusive(TO_INCLUSIVE),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-lhs"}}}
    /// Navigate to [`Expression`] across R58(1-*c)
    pub async fn r58_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r58_expression");
        match self.lhs {
            Some(ref lhs) => vec![store.exhume_expression(lhs).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-rhs"}}}
    /// Navigate to [`Expression`] across R59(1-*c)
    pub async fn r59_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r59_expression");
        match self.rhs {
            Some(ref rhs) => vec![store.exhume_expression(rhs).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-impl-nav-subtype-to-supertype-expression"}}}
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
                if let ExpressionEnum::RangeExpression(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-implementation"}}}
impl PartialEq for RangeExpression {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.lhs == other.lhs && self.rhs == other.rhs
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
