// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"range_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::from::FROM;
use crate::v2::lu_dog_async::types::full::FULL;
use crate::v2::lu_dog_async::types::inclusive::INCLUSIVE;
use crate::v2::lu_dog_async::types::to::TO;
use crate::v2::lu_dog_async::types::to_inclusive::TO_INCLUSIVE;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RangeExpression {
    pub subtype: RangeExpressionEnum,
    pub id: Uuid,
    /// R58: [`RangeExpression`] '' [`Expression`]
    pub lhs: Option<Uuid>,
    /// R59: [`RangeExpression`] '' [`Expression`]
    pub rhs: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: futures::future::OptionFuture::from(
                lhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: RangeExpressionEnum::From(FROM),
            id,
        }));
        store.inter_range_expression(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_full"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_full(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: futures::future::OptionFuture::from(
                lhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: RangeExpressionEnum::Full(FULL),
            id,
        }));
        store.inter_range_expression(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: futures::future::OptionFuture::from(
                lhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: RangeExpressionEnum::Inclusive(INCLUSIVE),
            id,
        }));
        store.inter_range_expression(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_to(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: futures::future::OptionFuture::from(
                lhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: RangeExpressionEnum::To(TO),
            id,
        }));
        store.inter_range_expression(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub async fn new_to_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: futures::future::OptionFuture::from(
                lhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: RangeExpressionEnum::ToInclusive(TO_INCLUSIVE),
            id,
        }));
        store.inter_range_expression(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-lhs"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-rhs"}}}
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-lhs"}}}
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
        vec![store.exhume_expression(&self.id).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
