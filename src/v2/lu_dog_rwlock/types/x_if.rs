// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_if-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::block::Block;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-documentation"}}}
/// The if Expression
///
/// It does include an else, at no extra charge!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XIf {
    pub id: Uuid,
    /// R52: [`XIf`] 'false block' [`Expression`]
    pub false_block: Option<Uuid>,
    /// R44: [`XIf`] 'branches based on' [`Expression`]
    pub test: Uuid,
    /// R46: [`XIf`] 'when true, evaluates' [`Block`]
    pub true_block: Uuid,
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
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<XIf>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XIf {
            id,
            false_block: false_block.map(|expression| expression.read().unwrap().id),
            test: test.read().unwrap().id,
            true_block: true_block.read().unwrap().id,
        }));
        store.inter_x_if(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-cond-to-false_block"}}}
    /// Navigate to [`Expression`] across R52(1-*c)
    pub fn r52_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        match self.false_block {
            Some(ref false_block) => vec![store.exhume_expression(&false_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-true_block"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-test"}}}
    /// Navigate to [`Expression`] across R44(1-*)
    pub fn r44_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.test).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-true_block"}}}
    /// Navigate to [`Block`] across R46(1-*)
    pub fn r46_block<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Block>>> {
        vec![store.exhume_block(&self.true_block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
