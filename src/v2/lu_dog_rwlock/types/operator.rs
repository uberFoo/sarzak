// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::binary::Binary;
use crate::v2::lu_dog_rwlock::types::comparison::Comparison;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock::types::unary::Unary;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
    /// R50: [`Operator`] 'left hand side' [`Expression`]
    pub lhs: Uuid,
    /// R51: [`Operator`] 'right hand side' [`Expression`]
    pub rhs: Option<Uuid>,
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
    pub fn new_binary(
        lhs: &Arc<RwLock<Expression>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<Binary>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Operator {
            lhs: lhs.read().unwrap().id,
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: OperatorEnum::Binary(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_comparison"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub fn new_comparison(
        lhs: &Arc<RwLock<Expression>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<Comparison>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Operator {
            lhs: lhs.read().unwrap().id,
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: OperatorEnum::Comparison(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_unary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub fn new_unary(
        lhs: &Arc<RwLock<Expression>>,
        rhs: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<Unary>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Operator {
            lhs: lhs.read().unwrap().id,
            rhs: rhs.map(|expression| expression.read().unwrap().id),
            subtype: OperatorEnum::Unary(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    /// Navigate to [`Expression`] across R50(1-*)
    pub fn r50_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.lhs).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-cond-to-rhs"}}}
    /// Navigate to [`Expression`] across R51(1-*c)
    pub fn r51_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        match self.rhs {
            Some(ref rhs) => vec![store.exhume_expression(&rhs).unwrap()],
            None => Vec::new(),
        }
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Operator(id) = expression.read().unwrap().subtype {
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
