// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"print-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-documentation"}}}
/// A Print Expression?
///
/// Shold this be a statement?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Print {
    pub id: usize,
    /// R32: [`Print`] '' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-implementation"}}}
impl Print {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-impl-new"}}}
    /// Inter a new 'Print' in the store, and return it's `id`.
    pub fn new(
        expression: &Rc<RefCell<Expression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Print>> {
        store.inter_print(|id| {
            Rc::new(RefCell::new(Print {
                id,
                expression: expression.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R32(1-*)
    pub fn r32_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r32_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Print(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-implementation"}}}
impl PartialEq for Print {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
