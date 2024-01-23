// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::binary::Binary;
use crate::v2::lu_dog_async::types::comparison::Comparison;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Operator {
    pub subtype: OperatorEnum,
    pub id: usize,
    /// R50: [`Operator`] 'left hand side' [`Expression`]
    pub lhs: usize,
    /// R51: [`Operator`] 'right hand side' [`Expression`]
    pub rhs: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OperatorEnum {
    Binary(usize),
    Comparison(usize),
    Unary(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-implementation"}}}
impl Operator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_binary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub async fn new_binary(
        lhs: &Arc<RwLock<Expression>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<Binary>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Operator>> {
        let s_id = subtype.read().await.id;
        let lhs = lhs.read().await.id;
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_operator(|id| {
                Arc::new(RwLock::new(Operator {
                    lhs, // (b)
                    rhs, // (a)
                    subtype: OperatorEnum::Binary(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_comparison"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub async fn new_comparison(
        lhs: &Arc<RwLock<Expression>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<Comparison>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Operator>> {
        let s_id = subtype.read().await.id;
        let lhs = lhs.read().await.id;
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_operator(|id| {
                Arc::new(RwLock::new(Operator {
                    lhs, // (b)
                    rhs, // (a)
                    subtype: OperatorEnum::Comparison(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_unary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub async fn new_unary(
        lhs: &Arc<RwLock<Expression>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<Unary>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Operator>> {
        let s_id = subtype.read().await.id;
        let lhs = lhs.read().await.id;
        let rhs = match rhs {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_operator(|id| {
                Arc::new(RwLock::new(Operator {
                    lhs, // (b)
                    rhs, // (a)
                    subtype: OperatorEnum::Unary(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    /// Navigate to [`Expression`] across R50(1-*)
    pub async fn r50_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        stream::iter(vec![store.exhume_expression(&self.lhs).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-cond-to-rhs"}}}
    /// Navigate to [`Expression`] across R51(1-*c)
    pub async fn r51_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        match self.rhs {
            Some(ref rhs) => {
                stream::iter(vec![store.exhume_expression(rhs).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::Operator(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-implementation"}}}
impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.lhs == other.lhs && self.rhs == other.rhs
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
