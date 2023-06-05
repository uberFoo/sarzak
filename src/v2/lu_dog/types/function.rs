// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog::types::implementation::Implementation;
use crate::v2::lu_dog::types::item::Item;
use crate::v2::lu_dog::types::item::ItemEnum;
use crate::v2::lu_dog::types::parameter::Parameter;
use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-documentation"}}}
/// A Function
///
/// Inputs, outputs. Stuff happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Function {
    pub id: Uuid,
    pub name: String,
    /// R19: [`Function`] 'executes statements in a' [`Block`]
    pub block: Uuid,
    /// R9: [`Function`] 'may be contained in an' [`Implementation`]
    pub impl_block: Option<Uuid>,
    /// R10: [`Function`] 'returns' [`ValueType`]
    pub return_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new 'Function' in the store, and return it's `id`.
    pub fn new(
        name: String,
        block: &Rc<RefCell<Block>>,
        impl_block: Option<&Rc<RefCell<Implementation>>>,
        return_type: &Rc<RefCell<ValueType>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Function>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Function {
            id,
            name,
            block: block.borrow().id,
            impl_block: impl_block.map(|implementation| implementation.borrow().id),
            return_type: return_type.borrow().id(),
        }));
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R19(1-*)
    pub fn r19_block<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Block>>> {
        span!("r19_block");
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-impl_block"}}}
    /// Navigate to [`Implementation`] across R9(1-*c)
    pub fn r9_implementation<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<Implementation>>> {
        span!("r9_implementation");
        match self.impl_block {
            Some(ref impl_block) => vec![store.exhume_implementation(impl_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R10(1-*)
    pub fn r10_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r10_value_type");
        vec![store.exhume_value_type(&self.return_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R13(1-M)
    pub fn r13_parameter<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Parameter>>> {
        span!("r13_parameter");
        store
            .iter_parameter()
            .filter(|parameter| parameter.borrow().function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<FieldAccessTarget>>> {
        span!("r67_field_access_target");
        vec![store.exhume_field_access_target(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Function(id) = item.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
