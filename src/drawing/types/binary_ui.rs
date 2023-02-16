// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referrer imports
use crate::drawing::types::anchor::Anchor;
use crate::drawing::types::binary::Binary;

use crate::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-documentation"}}}
/// This represents additional information necessary to render a `Binary` relationship in the
/// user interface.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BinaryUi {
    pub id: Uuid,
    /// R7: [`BinaryUi`] 'anchors' [`Anchor`]
    pub from: Option<Uuid>,
    /// R8: [`BinaryUi`] 'anchors the to side' [`Anchor`]
    pub to: Option<Uuid>,
    /// R12: [`BinaryUi`] 'is represented in the UI by' [`Binary`]
    pub binary_id: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-implementation"}}}
impl BinaryUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-new"}}}
    /// Inter a new BinaryUi in the store, and return it's `id`.
    pub fn new(
        from: Option<&Anchor>,
        to: Option<&Anchor>,
        binary_id: Option<&Binary>,
        store: &mut DrawingStore,
    ) -> BinaryUi {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}:{:?}", from, to, binary_id).as_bytes(),
        );
        let new = BinaryUi {
            from: from.map(|anchor| anchor.id),
            to: to.map(|anchor| anchor.id),
            binary_id: binary_id.map(|binary| binary.id),
            id,
        };
        store.inter_binary_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-cond-to-from"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-cond-to-to"}}}
    /// Navigate to [`Anchor`] across R7(1-?c)
    pub fn anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        match self.from {
            Some(ref from) => vec![store.exhume_anchor(from).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-cond-to-from"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-cond-to-to"}}}
    /// Navigate to [`Anchor`] across R8(1-?c)
    pub fn anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        match self.to {
            Some(ref to) => vec![store.exhume_anchor(to).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-cond-to-binary_id"}}}
    /// Navigate to [`Binary`] across R12(1-?c)
    pub fn binary<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Binary> {
        match self.binary_id {
            Some(ref binary_id) => vec![store.exhume_binary(binary_id).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
