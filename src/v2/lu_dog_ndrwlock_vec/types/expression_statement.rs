// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_ndrwlock_vec::types::statement::Statement;
use crate::v2::lu_dog_ndrwlock_vec::types::statement::StatementEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-documentation"}}}
/// A statement that consists of just an expression.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExpressionStatement {
    pub id: usize,
    /// R31: [`ExpressionStatement`] '' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-implementation"}}}
impl ExpressionStatement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-impl-new"}}}
    /// Inter a new 'Expression Statement' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ExpressionStatement>> {
        store.inter_expression_statement(|id| {
            Arc::new(RwLock::new(ExpressionStatement {
                id,
                expression: expression.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R31(1-*)
    pub fn r31_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r31_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R16(isa)
    pub fn r16_statement<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r16_statement");
        vec![store
            .iter_statement()
            .find(|statement| {
                if let StatementEnum::ExpressionStatement(id) = statement.read().unwrap().subtype {
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
