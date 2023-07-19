// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"error_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-documentation"}}}
/// An Error...
///
/// I'm not sure what to do with this.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorExpression {
    pub id: usize,
    pub span: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-implementation"}}}
impl ErrorExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-impl-new"}}}
    /// Inter a new 'Error Expression' in the store, and return it's `id`.
    pub fn new(span: String, store: &mut LuDogRwlockVecStore) -> Arc<RwLock<ErrorExpression>> {
        store.inter_error_expression(|id| {
            Arc::new(RwLock::new(ErrorExpression {
                id,
                span: span.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ErrorExpression(id) = expression.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-implementation"}}}
impl PartialEq for ErrorExpression {
    fn eq(&self, other: &Self) -> bool {
        self.span == other.span
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
