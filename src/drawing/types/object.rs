// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referent imports
use crate::drawing::types::object_ui::ObjectUi;

use crate::drawing::store::ObjectStore as DrawingStore;
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
    pub fn new(name: String, store: &mut DrawingStore) -> Object {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Object { name: name, id };
        store.inter_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-one-to-object_ui"}}}
    /// Navigate to [`ObjectUi`] across R1(1-1)
    pub fn object_ui_r1<'a>(&'a self, store: &'a DrawingStore) -> Vec<&ObjectUi> {
        vec![
            store
                .iter_object_ui()
                .find(|object_ui| object_ui.1.object_id == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
