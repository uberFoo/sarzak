// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"import-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::item::Item;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-documentation"}}}
/// An Import of a foreign ObjectStore
///
/// This indicates to the downstream model compiler that it needs to emit code to load the imported
///  ObjectStore.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Import {
    pub id: Uuid,
    pub path: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-implementation"}}}
impl Import {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-impl-new"}}}
    /// Inter a new 'Import' in the store, and return it's `id`.
    pub fn new(path: String, store: &mut LuDogStore) -> Import {
        let id = Uuid::new_v4();
        let new = Import { id: id, path: path };
        store.inter_import(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-impl-new_"}}}
    /// Inter a new 'Import' in the store, and return it's `id`.
    pub fn new_(path: String) -> Import {
        let id = Uuid::new_v4();
        let new = Import { id: id, path: path };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Item> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
