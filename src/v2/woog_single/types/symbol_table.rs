// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"symbol_table-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog_single::types::block::Block;
use crate::v2::woog_single::types::variable::Variable;
use serde::{Deserialize, Serialize};

use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-struct-documentation"}}}
/// Local variable symbol table
///
/// This is how we know what's available, and what it's type is.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SymbolTable {
    pub id: Uuid,
    /// R24: [`SymbolTable`] 'co-exists with a' [`Block`]
    pub block: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-implementation"}}}
impl SymbolTable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-struct-impl-new"}}}
    /// Inter a new 'Symbol Table' in the store, and return it's `id`.
    pub fn new(block: &Block, store: &mut WoogSingleStore) -> SymbolTable {
        let id = Uuid::new_v4();
        let new = SymbolTable {
            id,
            block: block.id,
        };
        store.inter_symbol_table(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R24(1-*)
    pub fn r24_block<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"symbol_table-struct-impl-nav-backward-1_M-to-variable"}}}
    /// Navigate to [`Variable`] across R20(1-M)
    pub fn r20_variable<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Variable> {
        store
            .iter_variable()
            .filter(|variable| variable.symbol_table == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
