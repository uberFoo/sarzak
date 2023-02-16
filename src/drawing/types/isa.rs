// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referent imports
use crate::drawing::types::isa_ui::IsaUi;

use crate::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-documentation"}}}
/// The imported R_ISA object from the sarzak domain.
///
/// ❗️{ "imported_object": { "domain": "sarzak", "package": "sarzak", "model_path": "./
///" }}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Isa {
    pub id: Uuid,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-implementation"}}}
impl Isa {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-new"}}}
    /// Inter a new Isa in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut DrawingStore) -> Isa {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", number).as_bytes());
        let new = Isa { number: number, id };
        store.inter_isa(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-backward-one-to-isa_ui"}}}
    /// Navigate to [`IsaUi`] across R11(1-1)
    pub fn isa_ui_r11<'a>(&'a self, store: &'a DrawingStore) -> Vec<&IsaUi> {
        vec![
            store
                .iter_isa_ui()
                .find(|isa_ui| isa_ui.1.isa == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
