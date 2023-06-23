// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_object_store-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-documentation"}}}
/// A generated ObjectStore
///
/// This is the backing store for the structs.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ZObjectStore {
    pub domain: String,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-implementation"}}}
impl ZObjectStore {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-impl-new"}}}
    /// Inter a new 'Object Store' in the store, and return it's `id`.
    pub fn new(domain: String, store: &mut LuDogPlMutexStore) -> Arc<Mutex<ZObjectStore>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(ZObjectStore { domain, id }));
        store.inter_z_object_store(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<ValueType>>> {
        span!("r1_value_type");
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}