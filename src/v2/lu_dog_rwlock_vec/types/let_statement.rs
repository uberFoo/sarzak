// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"let_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::local_variable::LocalVariable;
use crate::v2::lu_dog_rwlock_vec::types::statement::Statement;
use crate::v2::lu_dog_rwlock_vec::types::statement::StatementEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-documentation"}}}
/// A Let Statement
///
/// This statement assigns a value from an expression to a local variable.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LetStatement {
    pub id: usize,
    /// R20: [`LetStatement`] 'assigns the value of an' [`Expression`]
    pub expression: usize,
    /// R21: [`LetStatement`] 'assigns a value to a' [`LocalVariable`]
    pub variable: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-implementation"}}}
impl LetStatement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-impl-new"}}}
    /// Inter a new 'Let Statement' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<RwLock<Expression>>,
        variable: &Arc<RwLock<LocalVariable>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<LetStatement>> {
        store.inter_let_statement(|id| {
            Arc::new(RwLock::new(LetStatement {
                id,
                expression: expression.read().unwrap().id,
                variable: variable.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R20(1-*)
    pub fn r20_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-impl-nav-forward-to-variable"}}}
    /// Navigate to [`LocalVariable`] across R21(1-*)
    pub fn r21_local_variable<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<LocalVariable>>> {
        vec![store.exhume_local_variable(&self.variable).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R16(isa)
    pub fn r16_statement<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        vec![store
            .iter_statement()
            .find(|statement| {
                if let StatementEnum::LetStatement(id) = statement.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-implementation"}}}
impl PartialEq for LetStatement {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression && self.variable == other.variable
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
