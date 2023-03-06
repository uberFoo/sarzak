// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::object_method::ObjectMethod;
use crate::v2::woog::types::statement::Statement;
use crate::v2::woog::types::symbol_table::SymbolTable;
use crate::v2::woog::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-documentation"}}}
/// A Block Expression
///
/// This is a block of code, you know, between `{` and `}`. See the [reference](https://doc.rust
///-lang.org/reference/expressions/block-expr.html).
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub id: Uuid,
    pub instance: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new(instance: Uuid, store: &mut WoogStore) -> Block {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", instance).as_bytes());
        let new = Block {
            instance: instance,
            id,
        };
        store.inter_block(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-object_method"}}}
    /// Navigate to [`ObjectMethod`] across R23(1-1c)
    pub fn r23c_object_method<'a>(&'a self, store: &'a WoogStore) -> Vec<&ObjectMethod> {
        let object_method = store
            .iter_object_method()
            .find(|object_method| object_method.block == self.id);
        match object_method {
            Some(ref object_method) => vec![object_method],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R12(1-M)
    pub fn r12_statement<'a>(&'a self, store: &'a WoogStore) -> Vec<&Statement> {
        store
            .iter_statement()
            .filter_map(|statement| {
                if statement.block == self.id {
                    Some(statement)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-to-symbol_table"}}}
    /// Navigate to [`SymbolTable`] across R24(1-1)
    pub fn r24_symbol_table<'a>(&'a self, store: &'a WoogStore) -> Vec<&SymbolTable> {
        vec![store
            .iter_symbol_table()
            .find(|symbol_table| symbol_table.block == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-to-symbol_table"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R10(isa)
    pub fn r10_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
