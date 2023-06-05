// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::function::Function;
use crate::v2::lu_dog_pl_mutex::types::item::Item;
use crate::v2::lu_dog_pl_mutex::types::item::ItemEnum;
use crate::v2::lu_dog_pl_mutex::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Implementation {
    pub id: Uuid,
    /// R8: [`Implementation`] 'adds functions to a' [`WoogStruct`]
    pub model_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-implementation"}}}
impl Implementation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-new"}}}
    /// Inter a new 'Implementation' in the store, and return it's `id`.
    pub fn new(
        model_type: &Arc<Mutex<WoogStruct>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Implementation>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Implementation {
            id,
            model_type: model_type.lock().id,
        }));
        store.inter_implementation(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-nav-forward-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*)
    pub fn r8_woog_struct<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<WoogStruct>>> {
        span!("r8_woog_struct");
        vec![store.exhume_woog_struct(&self.model_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub fn r9_function<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Function>>> {
        span!("r9_function");
        store
            .iter_function()
            .filter_map(|function| {
                if function.lock().impl_block == Some(self.id) {
                    Some(function)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Implementation(id) = item.lock().subtype {
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
