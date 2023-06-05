// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"local_variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::let_statement::LetStatement;
use crate::v2::lu_dog_rwlock::types::variable::Variable;
use crate::v2::lu_dog_rwlock::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-implementation"}}}
impl LocalVariable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-impl-new"}}}
    /// Inter a new 'Local Variable' in the store, and return it's `id`.
    pub fn new(bug: Uuid, store: &mut LuDogRwlockStore) -> Arc<RwLock<LocalVariable>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(LocalVariable { bug, id }));
        store.inter_local_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-impl-nav-backward-one-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R21(1-1)
    pub fn r21_let_statement<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<LetStatement>>> {
        span!("r21_let_statement");
        vec![store
            .iter_let_statement()
            .find(|let_statement| let_statement.read().unwrap().variable == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub fn r12_variable<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Variable>>> {
        span!("r12_variable");
        vec![store
            .iter_variable()
            .find(|variable| {
                if let VariableEnum::LocalVariable(id) = variable.read().unwrap().subtype {
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
