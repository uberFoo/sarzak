// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::binary::Binary;
use crate::v2::lu_dog_async::types::comparison::Comparison;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::unary::Unary;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-documentation"}}}
/// Operator Expressions
///
/// Basically anything you can do with an expression is a subtype of this beasty.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Operator {
    pub subtype: OperatorEnum,
    pub id: Uuid,
    /// R51: [`Operator`] 'right hand side' [`Expression`]
    pub rhs: Option<Uuid>,
    /// R50: [`Operator`] 'left hand side' [`Expression`]
    pub lhs: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum OperatorEnum {
    Binary(Uuid),
    Comparison(Uuid),
    Unary(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-implementation"}}}
impl Operator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_binary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub async fn new_binary(
        rhs: Option<&Arc<RwLock<Expression>>>,
        lhs: &Arc<RwLock<Expression>>,
        subtype: &Arc<RwLock<Binary>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Operator {
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            lhs: lhs.read().await.id(),
            subtype: OperatorEnum::Binary(subtype.read().await.id()),
            id,
        }));
        store.inter_operator(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_comparison"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub async fn new_comparison(
        rhs: Option<&Arc<RwLock<Expression>>>,
        lhs: &Arc<RwLock<Expression>>,
        subtype: &Arc<RwLock<Comparison>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Operator {
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            lhs: lhs.read().await.id(),
            subtype: OperatorEnum::Comparison(subtype.read().await.id()),
            id,
        }));
        store.inter_operator(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_unary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub async fn new_unary(
        rhs: Option<&Arc<RwLock<Expression>>>,
        lhs: &Arc<RwLock<Expression>>,
        subtype: &Arc<RwLock<Unary>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Operator {
            rhs: futures::future::OptionFuture::from(
                rhs.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            lhs: lhs.read().await.id(),
            subtype: OperatorEnum::Unary(subtype.read().await.id()),
            id,
        }));
        store.inter_operator(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-cond-to-rhs"}}}
    /// Navigate to [`Expression`] across R51(1-*c)
    pub async fn r51_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r51_expression");
        match self.rhs {
            Some(ref rhs) => vec![store.exhume_expression(rhs).await.unwrap()],
            None => Vec::new(),
        }
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    /// Navigate to [`Expression`] across R50(1-*)
    pub async fn r50_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r50_expression");
        vec![store.exhume_expression(&self.lhs).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-impl-nav-subtype-to-supertype-expression"}}}
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
