// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"grouped-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-struct-documentation"}}}
/// Parens
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Grouped {
    pub id: usize,
    /// R61: [`Grouped`] '' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-implementation"}}}
impl Grouped {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-struct-impl-new"}}}
    /// Inter a new 'Grouped' in the store, and return it's `id`.
    pub fn new(
        expression: &Rc<RefCell<Expression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Grouped>> {
        store.inter_grouped(|id| {
            Rc::new(RefCell::new(Grouped {
                id,
                expression: expression.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R61(1-*)
    pub fn r61_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r61_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Grouped(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grouped-implementation"}}}
impl PartialEq for Grouped {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
