// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation_block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::function::Function;
use crate::v2::lu_dog_ndrwlock_vec::types::item::Item;
use crate::v2::lu_dog_ndrwlock_vec::types::item::ItemEnum;
use crate::v2::lu_dog_ndrwlock_vec::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImplementationBlock {
    pub id: usize,
    /// R8: [`ImplementationBlock`] 'adds functions to a' [`WoogStruct`]
    pub model_type: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-implementation"}}}
impl ImplementationBlock {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-new"}}}
    /// Inter a new 'Implementation Block' in the store, and return it's `id`.
    pub fn new(
        model_type: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ImplementationBlock>> {
        store.inter_implementation_block(|id| {
            Arc::new(RwLock::new(ImplementationBlock {
                id,
                model_type: model_type.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*)
    pub fn r8_woog_struct<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<WoogStruct>>> {
        span!("r8_woog_struct");
        vec![store.exhume_woog_struct(&self.model_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub fn r9_function<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        span!("r9_function");
        store
            .iter_function()
            .filter(|function| function.read().unwrap().impl_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::ImplementationBlock(id) = item.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-implementation"}}}
impl PartialEq for ImplementationBlock {
    fn eq(&self, other: &Self) -> bool {
        self.model_type == other.model_type
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
