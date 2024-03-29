// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"range_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock::types::from::FROM;
use crate::v2::lu_dog_rwlock::types::full::FULL;
use crate::v2::lu_dog_rwlock::types::inclusive::INCLUSIVE;
use crate::v2::lu_dog_rwlock::types::to::TO;
use crate::v2::lu_dog_rwlock::types::to_inclusive::TO_INCLUSIVE;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
    pub fn new_from(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: lhs.map(|expression| expression.read().unwrap().id),
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: RangeExpressionEnum::From(FROM),
            id,
        }));
        store.inter_range_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_full"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_full(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: lhs.map(|expression| expression.read().unwrap().id),
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: RangeExpressionEnum::Full(FULL),
            id,
        }));
        store.inter_range_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: lhs.map(|expression| expression.read().unwrap().id),
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: RangeExpressionEnum::Inclusive(INCLUSIVE),
            id,
        }));
        store.inter_range_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_to(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: lhs.map(|expression| expression.read().unwrap().id),
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: RangeExpressionEnum::To(TO),
            id,
        }));
        store.inter_range_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_to_inclusive(
        lhs: Option<&Arc<RwLock<Expression>>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<RangeExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RangeExpression {
            lhs: lhs.map(|expression| expression.read().unwrap().id),
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: RangeExpressionEnum::ToInclusive(TO_INCLUSIVE),
            id,
        }));
        store.inter_range_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-nav-forward-cond-to-lhs"}}}
    /// Navigate to [`Expression`] across R58(1-*c)
    pub fn r58_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
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
        store: &'a LuDogRwlockStore,
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
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::RangeExpression(id) = expression.read().unwrap().subtype {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
