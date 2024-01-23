// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"generation_unit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::sarzak::types::object::Object;
use crate::v2::woog::types::time_stamp::TimeStamp;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GenerationUnit {
    pub id: Uuid,
    /// R21: [`GenerationUnit`] 'is created at' [`TimeStamp`]
    pub creation_time: Uuid,
    /// R22: [`GenerationUnit`] 'is based on an' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-implementation"}}}
impl GenerationUnit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-struct-impl-new"}}}
    /// Inter a new 'Generation Unit' in the store, and return it's `id`.
    pub fn new(
        creation_time: &Arc<RwLock<TimeStamp>>,
        object: &Object,
        store: &mut WoogStore,
    ) -> Arc<RwLock<GenerationUnit>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(GenerationUnit {
            id,
            creation_time: creation_time.read().unwrap().id,
            object: object.id,
        }));
        store.inter_generation_unit(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-struct-impl-nav-forward-to-creation_time"}}}
    /// Navigate to [`TimeStamp`] across R21(1-*)
    pub fn r21_time_stamp<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<TimeStamp>>> {
        vec![store.exhume_time_stamp(&self.creation_time).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R22(1-*)
    pub fn r22_object<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<Object>>> {
        vec![store.exhume_object(&self.object).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generation_unit-struct-impl-nav-forward-to-creation_time"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
