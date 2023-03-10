// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::block::Block;
use crate::v2::woog::types::call::Call;
use crate::v2::woog::types::closure::CLOSURE;
use crate::v2::woog::types::item::Item;
use crate::v2::woog::types::object_method::ObjectMethod;
use crate::v2::woog::types::parameter::Parameter;
use crate::v2::woog::types::plain_old_function::PLAIN_OLD_FUNCTION;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-documentation"}}}
/// A function
///
/// This is a plain old function, distinct from an [`Object Method`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Function {
    pub subtype: FunctionEnum,
    pub description: String,
    pub id: Uuid,
    pub name: String,
    /// R23: [`Function`] 'contains' [`Block`]
    pub block: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FunctionEnum {
    Closure(Uuid),
    ObjectMethod(Uuid),
    PlainOldFunction(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new Function in the store, and return it's `id`.
    pub fn new_closure(
        description: String,
        name: String,
        block: &Block,
        store: &mut WoogStore,
    ) -> Function {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = CLOSURE;
        let new = Function {
            description: description,
            name: name,
            block: block.id,
            subtype: FunctionEnum::Closure(CLOSURE),
            id,
        };
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new Function in the store, and return it's `id`.
    pub fn new_object_method(
        description: String,
        name: String,
        block: &Block,
        subtype: &ObjectMethod,
        store: &mut WoogStore,
    ) -> Function {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Function {
            description: description,
            name: name,
            block: block.id,
            subtype: FunctionEnum::ObjectMethod(subtype.id),
            id,
        };
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new Function in the store, and return it's `id`.
    pub fn new_plain_old_function(
        description: String,
        name: String,
        block: &Block,
        store: &mut WoogStore,
    ) -> Function {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = PLAIN_OLD_FUNCTION;
        let new = Function {
            description: description,
            name: name,
            block: block.id,
            subtype: FunctionEnum::PlainOldFunction(PLAIN_OLD_FUNCTION),
            id,
        };
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R23(1-*)
    pub fn r23_block<'a>(&'a self, store: &'a WoogStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-call"}}}
    /// Navigate to [`Call`] across R19(1-M)
    pub fn r19_call<'a>(&'a self, store: &'a WoogStore) -> Vec<&Call> {
        store
            .iter_call()
            .filter_map(|call| {
                if call.function == self.id {
                    Some(call)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R5(1c-1c)
    pub fn r5c_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.function == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R26(isa)
    pub fn r26_item<'a>(&'a self, store: &'a WoogStore) -> Vec<&Item> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
