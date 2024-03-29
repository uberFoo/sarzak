// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"index-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::expression::Expression;
use crate::v2::lu_dog_pl_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-struct-documentation"}}}
/// An index expression
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Index {
    pub id: usize,
    /// R56: [`Index`] '' [`Expression`]
    pub index: usize,
    /// R57: [`Index`] '' [`Expression`]
    pub target: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-implementation"}}}
impl Index {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-struct-impl-new"}}}
    /// Inter a new 'Index' in the store, and return it's `id`.
    pub fn new(
        index: &Arc<RwLock<Expression>>,
        target: &Arc<RwLock<Expression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Index>> {
        store.inter_index(|id| {
            Arc::new(RwLock::new(Index {
                id,
                index: index.read().id,
                target: target.read().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-struct-impl-nav-forward-to-index"}}}
    /// Navigate to [`Expression`] across R56(1-*)
    pub fn r56_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.index).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-struct-impl-nav-forward-to-target"}}}
    /// Navigate to [`Expression`] across R57(1-*)
    pub fn r57_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.target).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Index(id) = expression.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"index-implementation"}}}
impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.target == other.target
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
