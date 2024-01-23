// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::body::Body;
use crate::v2::lu_dog_vec::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_vec::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_vec::types::func_generic::FuncGeneric;
use crate::v2::lu_dog_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vec::types::item::Item;
use crate::v2::lu_dog_vec::types::item::ItemEnum;
use crate::v2::lu_dog_vec::types::parameter::Parameter;
use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-documentation"}}}
/// A Function
///
/// Inputs, outputs. Stuff happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    pub id: usize,
    pub name: String,
    /// R19: [`Function`] 'executes statements in a' [`Body`]
    pub body: usize,
    /// R99: [`Function`] '' [`FuncGeneric`]
    pub first_generic: Option<usize>,
    /// R82: [`Function`] 'may have a first parameter' [`Parameter`]
    pub first_param: Option<usize>,
    /// R9: [`Function`] 'may be contained in an' [`ImplementationBlock`]
    pub impl_block: Option<usize>,
    /// R10: [`Function`] 'returns' [`ValueType`]
    pub return_type: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new 'Function' in the store, and return it's `id`.
    pub fn new(
        name: String,
        body: &Rc<RefCell<Body>>,
        first_generic: Option<&Rc<RefCell<FuncGeneric>>>,
        first_param: Option<&Rc<RefCell<Parameter>>>,
        impl_block: Option<&Rc<RefCell<ImplementationBlock>>>,
        return_type: &Rc<RefCell<ValueType>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Function>> {
        store.inter_function(|id| {
            Rc::new(RefCell::new(Function {
                id,
                name: name.to_owned(),
                body: body.borrow().id,
                first_generic: first_generic.map(|func_generic| func_generic.borrow().id),
                first_param: first_param.map(|parameter| parameter.borrow().id),
                impl_block: impl_block.map(|implementation_block| implementation_block.borrow().id),
                return_type: return_type.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-body"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-body"}}}
    /// Navigate to [`Body`] across R19(1-*)
    pub fn r19_body<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Body>>> {
        vec![store.exhume_body(&self.body).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-first_generic"}}}
    /// Navigate to [`FuncGeneric`] across R99(1-*c)
    pub fn r99_func_generic<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FuncGeneric>>> {
        match self.first_generic {
            Some(ref first_generic) => vec![store.exhume_func_generic(&first_generic).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-first_param"}}}
    /// Navigate to [`Parameter`] across R82(1-*c)
    pub fn r82_parameter<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Parameter>>> {
        match self.first_param {
            Some(ref first_param) => vec![store.exhume_parameter(&first_param).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-impl_block"}}}
    /// Navigate to [`ImplementationBlock`] across R9(1-*c)
    pub fn r9_implementation_block<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ImplementationBlock>>> {
        match self.impl_block {
            Some(ref impl_block) => vec![store.exhume_implementation_block(&impl_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R10(1-*)
    pub fn r10_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store.exhume_value_type(&self.return_type).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-function_call"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-func_generic"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_Mc-to-func_generic"}}}
    /// Navigate to [`FuncGeneric`] across R107(1-Mc)
    pub fn r107_func_generic<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FuncGeneric>>> {
        store
            .iter_func_generic()
            .filter(|func_generic| func_generic.borrow().func == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R13(1-M)
    pub fn r13_parameter<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Parameter>>> {
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
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FieldAccessTarget>>> {
        vec![store
            .iter_field_access_target()
            .find(|field_access_target| {
                if let FieldAccessTargetEnum::Function(id) = field_access_target.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Item>>> {
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
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Function(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.body == other.body
            && self.first_generic == other.first_generic
            && self.first_param == other.first_param
            && self.impl_block == other.impl_block
            && self.return_type == other.return_type
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
