// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"await-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Await {
    pub id: usize,
    /// R98: [`Await`] 'awaits' [`Expression`]
    pub future: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-implementation"}}}
impl Await {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-struct-impl-new"}}}
    /// Inter a new 'Await' in the store, and return it's `id`.
    pub fn new(future: &Rc<RefCell<Expression>>, store: &mut LuDogVecStore) -> Rc<RefCell<Await>> {
        store.inter_await(|id| {
            Rc::new(RefCell::new(Await {
                id,
                future: future.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-struct-impl-nav-forward-to-future"}}}
    /// Navigate to [`Expression`] across R98(1-*)
    pub fn r98_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r98_expression");
        vec![store.exhume_expression(&self.future).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Await(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"await-implementation"}}}
impl PartialEq for Await {
    fn eq(&self, other: &Self) -> bool {
        self.future == other.future
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
