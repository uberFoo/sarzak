// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"range_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::expression::Expression;
use crate::v2::lu_dog_pl_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_pl_vec::types::from::FROM;
use crate::v2::lu_dog_pl_vec::types::full::FULL;
use crate::v2::lu_dog_pl_vec::types::inclusive::INCLUSIVE;
use crate::v2::lu_dog_pl_vec::types::to::TO;
use crate::v2::lu_dog_pl_vec::types::to_inclusive::TO_INCLUSIVE;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
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
    pub fn new_from(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<RangeExpression>> {
        store.inter_range_expression(|id| {
            Arc::new(RwLock::new(RangeExpression {
                lhs: lhs.map(|expression| expression.read().id),
                rhs: rhs.map(|expression| expression.read().id),
                subtype: RangeExpressionEnum::From(FROM),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_full"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_full(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<RangeExpression>> {
        store.inter_range_expression(|id| {
            Arc::new(RwLock::new(RangeExpression {
                lhs: lhs.map(|expression| expression.read().id),
                rhs: rhs.map(|expression| expression.read().id),
                subtype: RangeExpressionEnum::Full(FULL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<RangeExpression>> {
        store.inter_range_expression(|id| {
            Arc::new(RwLock::new(RangeExpression {
                lhs: lhs.map(|expression| expression.read().id),
                rhs: rhs.map(|expression| expression.read().id),
                subtype: RangeExpressionEnum::Inclusive(INCLUSIVE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_to(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<RangeExpression>> {
        store.inter_range_expression(|id| {
            Arc::new(RwLock::new(RangeExpression {
                lhs: lhs.map(|expression| expression.read().id),
                rhs: rhs.map(|expression| expression.read().id),
                subtype: RangeExpressionEnum::To(TO),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_to_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<RangeExpression>> {
        store.inter_range_expression(|id| {
            Arc::new(RwLock::new(RangeExpression {
                lhs: lhs.map(|expression| expression.read().id),
                rhs: rhs.map(|expression| expression.read().id),
                subtype: RangeExpressionEnum::ToInclusive(TO_INCLUSIVE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-lhs"}}}
    /// Navigate to [`Expression`] across R58(1-*c)
    pub fn r58_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        match self.lhs {
            Some(ref lhs) => vec![store.exhume_expression(&lhs).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-rhs"}}}
    /// Navigate to [`Expression`] across R59(1-*c)
    pub fn r59_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        match self.rhs {
            Some(ref rhs) => vec![store.exhume_expression(&rhs).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::RangeExpression(id) = expression.read().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
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
