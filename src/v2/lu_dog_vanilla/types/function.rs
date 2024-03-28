// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::body::Body;
use crate::v2::lu_dog_vanilla::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_vanilla::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_vanilla::types::func_generic::FuncGeneric;
use crate::v2::lu_dog_vanilla::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vanilla::types::item::Item;
use crate::v2::lu_dog_vanilla::types::item::ItemEnum;
use crate::v2::lu_dog_vanilla::types::parameter::Parameter;
use crate::v2::lu_dog_vanilla::types::value_type::ValueType;
use crate::v2::lu_dog_vanilla::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
    /// R19: [`Function`] 'executes statements in a' [`Body`]
    pub body: Uuid,
    /// R99: [`Function`] '' [`FuncGeneric`]
    pub first_generic: Option<Uuid>,
    /// R82: [`Function`] 'may have a first parameter' [`Parameter`]
    pub first_param: Option<Uuid>,
    /// R9: [`Function`] 'may be contained in an' [`ImplementationBlock`]
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
        body: &Body,
        first_generic: Option<&FuncGeneric>,
        first_param: Option<&Parameter>,
        impl_block: Option<&ImplementationBlock>,
        return_type: &ValueType,
        store: &mut LuDogVanillaStore,
    ) -> Function {
        let id = Uuid::new_v4();
        let new = Function {
            id,
            name,
            body: body.id,
            first_generic: first_generic.as_ref().map(|func_generic| func_generic.id),
            first_param: first_param.as_ref().map(|parameter| parameter.id),
            impl_block: impl_block
                .as_ref()
                .map(|implementation_block| implementation_block.id),
            return_type: return_type.id,
        };
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-body"}}}
    /// Navigate to [`Body`] across R19(1-*)
    pub fn r19_body<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Body> {
        vec![store.exhume_body(&self.body).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-first_generic"}}}
    /// Navigate to [`FuncGeneric`] across R99(1-*c)
    pub fn r99_func_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FuncGeneric> {
        match self.first_generic {
            Some(ref first_generic) => vec![store.exhume_func_generic(first_generic).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-first_param"}}}
    /// Navigate to [`Parameter`] across R82(1-*c)
    pub fn r82_parameter<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Parameter> {
        match self.first_param {
            Some(ref first_param) => vec![store.exhume_parameter(first_param).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-impl_block"}}}
    /// Navigate to [`ImplementationBlock`] across R9(1-*c)
    pub fn r9_implementation_block<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&ImplementationBlock> {
        match self.impl_block {
            Some(ref impl_block) => vec![store.exhume_implementation_block(impl_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R10(1-*)
    pub fn r10_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.return_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-func_generic"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_Mc-to-func_generic"}}}
    /// Navigate to [`FuncGeneric`] across R107(1-Mc)
    pub fn r107_func_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FuncGeneric> {
        store
            .iter_func_generic()
            .filter(|func_generic| func_generic.func == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R13(1-M)
    pub fn r13_parameter<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Parameter> {
        store
            .iter_parameter()
            .filter(|parameter| parameter.function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&FieldAccessTarget> {
        vec![store
            .iter_field_access_target()
            .find(|field_access_target| {
                if let FieldAccessTargetEnum::Function(id) = field_access_target.subtype {
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
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Item> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Function(id) = item.subtype {
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
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Function(id) = value_type.subtype {
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