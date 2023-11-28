// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"range_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::expression::Expression;
use crate::v2::lu_dog_vec_tracy::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec_tracy::types::from::FROM;
use crate::v2::lu_dog_vec_tracy::types::full::FULL;
use crate::v2::lu_dog_vec_tracy::types::inclusive::INCLUSIVE;
use crate::v2::lu_dog_vec_tracy::types::to::TO;
use crate::v2::lu_dog_vec_tracy::types::to_inclusive::TO_INCLUSIVE;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
        lhs: Option<&Rc<RefCell<Expression>>>,
        rhs: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<RangeExpression>> {
        store.inter_range_expression(|id| {
            Rc::new(RefCell::new(RangeExpression {
                lhs: lhs.map(|expression| expression.borrow().id),
                rhs: rhs.map(|expression| expression.borrow().id),
                subtype: RangeExpressionEnum::From(FROM),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_full"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_full(
        lhs: Option<&Rc<RefCell<Expression>>>,
        rhs: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<RangeExpression>> {
        store.inter_range_expression(|id| {
            Rc::new(RefCell::new(RangeExpression {
                lhs: lhs.map(|expression| expression.borrow().id),
                rhs: rhs.map(|expression| expression.borrow().id),
                subtype: RangeExpressionEnum::Full(FULL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_inclusive(
        lhs: Option<&Rc<RefCell<Expression>>>,
        rhs: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<RangeExpression>> {
        store.inter_range_expression(|id| {
            Rc::new(RefCell::new(RangeExpression {
                lhs: lhs.map(|expression| expression.borrow().id),
                rhs: rhs.map(|expression| expression.borrow().id),
                subtype: RangeExpressionEnum::Inclusive(INCLUSIVE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_to(
        lhs: Option<&Rc<RefCell<Expression>>>,
        rhs: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<RangeExpression>> {
        store.inter_range_expression(|id| {
            Rc::new(RefCell::new(RangeExpression {
                lhs: lhs.map(|expression| expression.borrow().id),
                rhs: rhs.map(|expression| expression.borrow().id),
                subtype: RangeExpressionEnum::To(TO),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range_expression-struct-impl-new_to_inclusive"}}}
    /// Inter a new RangeExpression in the store, and return it's `id`.
    pub fn new_to_inclusive(
        lhs: Option<&Rc<RefCell<Expression>>>,
        rhs: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<RangeExpression>> {
        store.inter_range_expression(|id| {
            Rc::new(RefCell::new(RangeExpression {
                lhs: lhs.map(|expression| expression.borrow().id),
                rhs: rhs.map(|expression| expression.borrow().id),
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r58_expression");
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r59_expression");
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::RangeExpression(id) = expression.borrow().subtype {
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
