// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_if-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::block::Block;
use crate::v2::lu_dog_ndrwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_ndrwlock_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-documentation"}}}
/// The if Expression
///
/// It does include an else, at no extra charge!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XIf {
    pub id: usize,
    /// R52: [`XIf`] 'false block' [`Expression`]
    pub false_block: Option<usize>,
    /// R44: [`XIf`] 'branches based on' [`Expression`]
    pub test: usize,
    /// R46: [`XIf`] 'when true, evaluates' [`Block`]
    pub true_block: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-implementation"}}}
impl XIf {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-new"}}}
    /// Inter a new 'If' in the store, and return it's `id`.
    pub fn new(
        false_block: Option<&Arc<RwLock<Expression>>>,
        test: &Arc<RwLock<Expression>>,
        true_block: &Arc<RwLock<Block>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<XIf>> {
        store.inter_x_if(|id| {
            Arc::new(RwLock::new(XIf {
                id,
                false_block: false_block.map(|expression| expression.read().unwrap().id),
                test: test.read().unwrap().id,
                true_block: true_block.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-cond-to-false_block"}}}
    /// Navigate to [`Expression`] across R52(1-*c)
    pub fn r52_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        match self.false_block {
            Some(ref false_block) => vec![store.exhume_expression(&false_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-test"}}}
    /// Navigate to [`Expression`] across R44(1-*)
    pub fn r44_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.test).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-true_block"}}}
    /// Navigate to [`Block`] across R46(1-*)
    pub fn r46_block<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Block>>> {
        vec![store.exhume_block(&self.true_block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::XIf(id) = expression.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-implementation"}}}
impl PartialEq for XIf {
    fn eq(&self, other: &Self) -> bool {
        self.false_block == other.false_block
            && self.test == other.test
            && self.true_block == other.true_block
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
