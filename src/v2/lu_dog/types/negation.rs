// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"negation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-documentation"}}}
/// The unary minus
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Negation {
    pub id: Uuid,
    /// R70: [`Negation`] '' [`Expression`]
    pub expr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-implementation"}}}
impl Negation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-impl-new"}}}
    /// Inter a new 'Negation' in the store, and return it's `id`.
    pub fn new(expr: &Rc<RefCell<Expression>>, store: &mut LuDogStore) -> Rc<RefCell<Negation>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Negation {
            id,
            expr: expr.borrow().id(),
        }));
        store.inter_negation(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-impl-nav-forward-to-expr"}}}
    /// Navigate to [`Expression`] across R70(1-*)
    pub fn r70_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r70_expression");
        vec![store.exhume_expression(&self.expr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
