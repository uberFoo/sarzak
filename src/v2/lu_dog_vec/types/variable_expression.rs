// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-documentation"}}}
/// A Local Variable Expression
///
/// This is what happens when a variable is an r-value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VariableExpression {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-implementation"}}}
impl VariableExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-impl-new"}}}
    /// Inter a new 'Variable Expression' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogVecStore) -> Rc<RefCell<VariableExpression>> {
        store.inter_variable_expression(|id| {
            Rc::new(RefCell::new(VariableExpression {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::VariableExpression(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-implementation"}}}
impl PartialEq for VariableExpression {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
