// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::local::Local;
use crate::v2::woog::types::parameter::Parameter;
use crate::v2::woog::types::symbol_table::SymbolTable;
use crate::v2::woog::types::value::Value;
use crate::v2::woog::types::x_let::XLet;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-documentation"}}}
/// A Variable
///
/// Basically a name given to some memory.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Variable {
    pub subtype: VariableEnum,
    pub id: Uuid,
    pub name: String,
    /// R20: [`Variable`] 'exists in a' [`SymbolTable`]
    pub symbol_table: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VariableEnum {
    Local(Uuid),
    Parameter(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl Variable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_local"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_local(
        name: String,
        symbol_table: &SymbolTable,
        subtype: &Local,
        store: &mut WoogStore,
    ) -> Variable {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Variable {
            name: name,
            symbol_table: symbol_table.id,
            subtype: VariableEnum::Local(subtype.id),
            id,
        };
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_local_"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_local_(name: String, symbol_table: &SymbolTable, subtype: &Local) -> Variable {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Variable {
            name: name,
            symbol_table: symbol_table.id,
            subtype: VariableEnum::Local(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_parameter(
        name: String,
        symbol_table: &SymbolTable,
        subtype: &Parameter,
        store: &mut WoogStore,
    ) -> Variable {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Variable {
            name: name,
            symbol_table: symbol_table.id,
            subtype: VariableEnum::Parameter(subtype.id),
            id,
        };
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_parameter_"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_parameter_(
        name: String,
        symbol_table: &SymbolTable,
        subtype: &Parameter,
    ) -> Variable {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Variable {
            name: name,
            symbol_table: symbol_table.id,
            subtype: VariableEnum::Parameter(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-nav-forward-to-symbol_table"}}}
    /// Navigate to [`SymbolTable`] across R20(1-*)
    pub fn r20_symbol_table<'a>(&'a self, store: &'a WoogStore) -> Vec<&SymbolTable> {
        vec![store.exhume_symbol_table(&self.symbol_table).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-nav-backward-1_M-to-x_let"}}}
    /// Navigate to [`XLet`] across R17(1-M)
    pub fn r17_x_let<'a>(&'a self, store: &'a WoogStore) -> Vec<&XLet> {
        store
            .iter_x_let()
            .filter_map(|x_let| {
                if x_let.variable == self.id {
                    Some(x_let)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-impl-nav-subtype-to-supertype-value"}}}
    // Navigate to [`Value`] across R7(isa)
    pub fn r7_value<'a>(&'a self, store: &'a WoogStore) -> Vec<&Value> {
        vec![store.exhume_value(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
