// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::body::Body;
use crate::v2::lu_dog_rwlock_vec::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_rwlock_vec::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_rwlock_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_rwlock_vec::types::item::Item;
use crate::v2::lu_dog_rwlock_vec::types::item::ItemEnum;
use crate::v2::lu_dog_rwlock_vec::types::parameter::Parameter;
use crate::v2::lu_dog_rwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_rwlock_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-documentation"}}}
/// A Function
///
/// Inputs, outputs. Stuff happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    pub id: usize,
    pub name: String,
    /// R19: [`Function`] 'executes statements in a' [`Body`]
    pub body: usize,
    /// R9: [`Function`] 'may be contained in an' [`ImplementationBlock`]
    pub impl_block: Option<usize>,
    /// R10: [`Function`] 'returns' [`ValueType`]
    pub return_type: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-enum-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new_lambda"}}}
    /// Inter a new 'Function' in the store, and return it's `id`.
    pub fn new(
        name: String,
        body: &Arc<RwLock<Body>>,
        impl_block: Option<&Arc<RwLock<ImplementationBlock>>>,
        return_type: &Arc<RwLock<ValueType>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Function>> {
        store.inter_function(|id| {
            Arc::new(RwLock::new(Function {
                id,
                name: name.to_owned(),
                body: body.read().unwrap().id,
                impl_block: impl_block
                    .map(|implementation_block| implementation_block.read().unwrap().id),
                return_type: return_type.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-body"}}}
    /// Navigate to [`Body`] across R19(1-*)
    pub fn r19_body<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Body>>> {
        span!("r19_body");
        vec![store.exhume_body(&self.body).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-impl_block"}}}
    /// Navigate to [`ImplementationBlock`] across R9(1-*c)
    pub fn r9_implementation_block<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ImplementationBlock>>> {
        span!("r9_implementation_block");
        match self.impl_block {
            Some(ref impl_block) => vec![store.exhume_implementation_block(&impl_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R10(1-*)
    pub fn r10_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r10_value_type");
        vec![store.exhume_value_type(&self.return_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R13(1-M)
    pub fn r13_parameter<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r13_parameter");
        store
            .iter_parameter()
            .filter(|parameter| parameter.read().unwrap().function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        span!("r67_field_access_target");
        vec![store
            .iter_field_access_target()
            .find(|field_access_target| {
                if let FieldAccessTargetEnum::Function(id) =
                    field_access_target.read().unwrap().subtype
                {
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
    pub fn r6_item<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Function(id) = item.read().unwrap().subtype {
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
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Function(id) = value_type.read().unwrap().subtype {
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
            && self.impl_block == other.impl_block
            && self.return_type == other.return_type
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
