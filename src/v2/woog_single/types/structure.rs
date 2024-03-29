// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"structure-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog_single::types::item::Item;
use crate::v2::woog_single::types::structure_field::StructureField;
use serde::{Deserialize, Serialize};

use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
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
    pub fn new(name: String, store: &mut WoogSingleStore) -> Structure {
        let id = Uuid::new_v4();
        let new = Structure { id, name };
        store.inter_structure(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-struct-impl-nav-backward-assoc-many-to-structure_field"}}}
    /// Navigate to [`StructureField`] across R27(1-M)
    pub fn r27_structure_field<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&StructureField> {
        store
            .iter_structure_field()
            .filter(|structure_field| structure_field.field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R26(isa)
    pub fn r26_item<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Item> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
