// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"for_loop-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-documentation"}}}
/// A For Loop Expression
///
/// An expression that matches for IDENT in EXPRESSION BLOCK.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ForLoop {
    pub id: Uuid,
    pub ident: String,
    /// R43: [`ForLoop`] 'executes a' [`Block`]
    pub block: Uuid,
    /// R42: [`ForLoop`] 'iterates over an' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-implementation"}}}
impl ForLoop {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-new"}}}
    /// Inter a new 'For Loop' in the store, and return it's `id`.
    pub fn new(
        ident: String,
        block: &Arc<Mutex<Block>>,
        expression: &Arc<Mutex<Expression>>,
        store: &mut LuDogStore,
    ) -> Arc<Mutex<ForLoop>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(ForLoop {
            id,
            ident,
            block: block.lock().id,
            expression: expression.lock().id(),
        }));
        store.inter_for_loop(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R43(1-*)
    pub fn r43_block<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Block>>> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R42(1-*)
    pub fn r42_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Expression>>> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
