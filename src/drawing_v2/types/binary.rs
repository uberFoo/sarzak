// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::drawing_v2::UUID_NS;

// Referent imports
use crate::drawing_v2::types::binary_ui::BinaryUi;

use crate::drawing_v2::store::ObjectStore as DrawingV2Store;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-documentation"}}}
/// Binary Relationship from sarzak domain
///
/// ❗️{ "imported_object": { "domain": "sarzak", "package": "sarzak", "model_path": "./
///" }}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Binary {
    pub id: Uuid,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl Binary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut DrawingV2Store) -> Binary {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", number).as_bytes());
        let new = Binary { number: number, id };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-nav-backward-cond-to-binary_ui"}}}
    /// Navigate to [`BinaryUi`] across R12(1-1c)
    pub fn r12c_binary_ui<'a>(&'a self, store: &'a DrawingV2Store) -> Vec<&BinaryUi> {
        let binary_ui = store
            .iter_binary_ui()
            .find(|binary_ui| binary_ui.1.binary_id == self.id);
        match binary_ui {
            Some(ref binary_ui) => vec![binary_ui.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
