// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"local_variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::let_statement::LetStatement;
use crate::v2::lu_dog_vec::types::variable::Variable;
use crate::v2::lu_dog_vec::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-documentation"}}}
/// A Local Variable in a Block
///
/// Note that a variable is an "l-value", so it represents a specific memory location.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LocalVariable {
    pub bug: Uuid,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-implementation"}}}
impl LocalVariable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-impl-new"}}}
    /// Inter a new 'Local Variable' in the store, and return it's `id`.
    pub fn new(bug: Uuid, store: &mut LuDogVecStore) -> Rc<RefCell<LocalVariable>> {
        store.inter_local_variable(|id| Rc::new(RefCell::new(LocalVariable { bug, id })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-impl-nav-backward-one-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R21(1-1)
    pub fn r21_let_statement<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<LetStatement>>> {
        span!("r21_let_statement");
        vec![store
            .iter_let_statement()
            .find(|let_statement| let_statement.borrow().variable == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub fn r12_variable<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Variable>>> {
        span!("r12_variable");
        vec![store
            .iter_variable()
            .find(|variable| {
                if let VariableEnum::LocalVariable(id) = variable.borrow().subtype {
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
