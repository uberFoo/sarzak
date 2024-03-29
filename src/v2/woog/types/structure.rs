// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"structure-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::item::Item;
use crate::v2::woog::types::structure_field::StructureField;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-struct-documentation"}}}
/// A Data Type
///
/// A structure is a chunk of memory with named, and typed fields.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Structure {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-implementation"}}}
impl Structure {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-struct-impl-new"}}}
    /// Inter a new 'Structure' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut WoogStore) -> Arc<RwLock<Structure>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Structure { id, name }));
        store.inter_structure(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-struct-impl-nav-backward-assoc-many-to-structure_field"}}}
    /// Navigate to [`StructureField`] across R27(1-M)
    pub fn r27_structure_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<StructureField>>> {
        store
            .iter_structure_field()
            .filter(|structure_field| structure_field.read().unwrap().field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R26(isa)
    pub fn r26_item<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Item>>> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
