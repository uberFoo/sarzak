// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_wrapper-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-documentation"}}}
/// The purpose of this object is to wrap `Object`. We need to be able to store a referential
///  attribute to the `ObjectStore`, and we can’t/don’t want to add that to `Object`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectWrapper {
    pub id: usize,
    /// R78: [`Object`] '🚧 Comments are out of order — see sarzak#14.' [`Object`]
    pub object: Uuid,
    /// R78: [`ZObjectStore`] '🚧 Comments are out of order — see sarzak#14.' [`ZObjectStore`]
    pub z_store: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-implementation"}}}
impl ObjectWrapper {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-impl-new"}}}
    /// Inter a new 'Object Wrapper' in the store, and return it's `id`.
    pub fn new(
        object: &Object,
        z_store: &Arc<RwLock<ZObjectStore>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ObjectWrapper>> {
        store.inter_object_wrapper(|id| {
            Arc::new(RwLock::new(ObjectWrapper {
                id,
                object: object.id,
                z_store: z_store.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-impl-nav-forward-assoc-to-object"}}}
    /// Navigate to [`Object`] across R78(1-*)
    pub fn r78_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-impl-nav-forward-assoc-to-z_store"}}}
    /// Navigate to [`ZObjectStore`] across R78(1-*)
    pub fn r78_z_object_store<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ZObjectStore>>> {
        span!("r78_z_object_store");
        vec![store.exhume_z_object_store(&self.z_store).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}