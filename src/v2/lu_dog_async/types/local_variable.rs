// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"local_variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::let_statement::LetStatement;
use crate::v2::lu_dog_async::types::variable::Variable;
use crate::v2::lu_dog_async::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    pub async fn new(bug: Uuid, store: &mut LuDogAsyncStore) -> Arc<RwLock<LocalVariable>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(LocalVariable { bug, id }));
        store.inter_local_variable(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-struct-impl-nav-backward-one-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R21(1-1)
    pub async fn r21_let_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<LetStatement>>> {
        span!("r21_let_statement");
        let mut result = Vec::new();
        for let_statement in store.iter_let_statement().await {
            if let_statement.read().await.variable == self.id {
                result.push(let_statement);
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local_variable-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub async fn r12_variable<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Variable>>> {
        span!("r12_variable");
        let mut result = Vec::new();
        for variable in store.iter_variable().await {
            if let VariableEnum::LocalVariable(id) = variable.read().await.subtype {
                result.push(variable.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
