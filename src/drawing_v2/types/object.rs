// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::drawing_v2::UUID_NS;

// Referent imports
use crate::drawing_v2::types::object_ui::ObjectUi;

use crate::drawing_v2::store::ObjectStore as DrawingV2Store;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-documentation"}}}
/// Object imported from the sarzak Domain.
///
/// We don’t have a means of representing this as imported in Cuckoo, so I’m just adding
/// it here.
///
/// ❗️{ "imported_object": { "domain": "sarzak", "package": "sarzak", "model_path": "./
///" }}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-implementation"}}}
impl Object {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-new"}}}
    /// Inter a new Object in the store, and return it's `id`.
    pub fn new(name: String, store: &mut DrawingV2Store) -> Object {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Object { name: name, id };
        store.inter_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-cond-to-object_ui"}}}
    /// Navigate to [`ObjectUi`] across R1(1-1c)
    pub fn r1c_object_ui<'a>(&'a self, store: &'a DrawingV2Store) -> Vec<&ObjectUi> {
        let object_ui = store
            .iter_object_ui()
            .find(|object_ui| object_ui.1.object_id == self.id);
        match object_ui {
            Some(ref object_ui) => vec![object_ui.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
