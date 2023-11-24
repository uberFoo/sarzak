// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"a_wait-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AWait {
    pub id: usize,
    /// R98: [`AWait`] 'awaits' [`Expression`]
    pub x_future: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-implementation"}}}
impl AWait {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-struct-impl-new"}}}
    /// Inter a new 'Await' in the store, and return it's `id`.
    pub fn new(
        x_future: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<AWait>> {
        store.inter_a_wait(|id| {
            Arc::new(RwLock::new(AWait {
                id,
                x_future: x_future.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-struct-impl-nav-forward-to-future"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-struct-impl-nav-forward-to-x_future"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R98(1-*)
    pub fn r98_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.x_future).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::AWait(id) = expression.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a_wait-implementation"}}}
impl PartialEq for AWait {
    fn eq(&self, other: &Self) -> bool {
        self.x_future == other.x_future
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
