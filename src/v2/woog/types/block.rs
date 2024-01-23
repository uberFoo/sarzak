// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::object_method::ObjectMethod;
use crate::v2::woog::types::statement::Statement;
use crate::v2::woog::types::symbol_table::SymbolTable;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-documentation"}}}
/// A Block Expression
///
/// This is a block of code, you know, between `{` and `}`. See the [reference](https://doc.rust
/// -lang.org/reference/expressions/block-expr.html).
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub id: Uuid,
    pub seed: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new(seed: Uuid, store: &mut WoogStore) -> Arc<RwLock<Block>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Block { id, seed }));
        store.inter_block(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-object_method"}}}
    /// Navigate to [`ObjectMethod`] across R23(1-1c)
    pub fn r23c_object_method<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<ObjectMethod>>> {
        let object_method = store
            .iter_object_method()
            .find(|object_method| object_method.read().unwrap().block == self.id);
        match object_method {
            Some(ref object_method) => vec![object_method.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R12(1-M)
    pub fn r12_statement<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Statement>>> {
        store
            .iter_statement()
            .filter(|statement| statement.read().unwrap().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-to-symbol_table"}}}
    /// Navigate to [`SymbolTable`] across R24(1-1)
    pub fn r24_symbol_table<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<SymbolTable>>> {
        vec![store
            .iter_symbol_table()
            .find(|symbol_table| symbol_table.read().unwrap().block == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R10(isa)
    pub fn r10_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
