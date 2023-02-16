// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referrer imports
use crate::drawing::types::anchor::Anchor;
use crate::drawing::types::isa::Isa;
use crate::drawing::types::subtype_anchors::SubtypeAnchors;

use crate::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-documentation"}}}
/// This represents additional data necessary to render an `Isa` relationship in the user interface
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IsaUi {
    pub id: Uuid,
    /// R9: [`IsaUi`] 'anchors' [`Anchor`]
    pub from: Option<Uuid>,
    /// R11: [`IsaUi`] 'is represented in the UI by' [`Isa`]
    pub isa: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-implementation"}}}
impl IsaUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-new"}}}
    /// Inter a new IsaUi in the store, and return it's `id`.
    pub fn new(from: Option<&Anchor>, isa: Option<&Isa>, store: &mut DrawingStore) -> IsaUi {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}:{:?}", from, isa).as_bytes());
        let new = IsaUi {
            from: from.map(|anchor| anchor.id),
            isa: isa.map(|isa| isa.id),
            id,
        };
        store.inter_isa_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-forward-cond-to-from"}}}
    /// Navigate to [`Anchor`] across R9(1-?c)
    pub fn anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        match self.from {
            Some(ref from) => vec![store.exhume_anchor(from).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-forward-cond-to-isa"}}}
    /// Navigate to [`Isa`] across R11(1-?c)
    pub fn isa<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Isa> {
        match self.isa {
            Some(ref isa) => vec![store.exhume_isa(isa).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-backward-assoc-one-cond-to-subtype_anchors"}}}
    /// Navigate to [`SubtypeAnchors`] across R10(1-1c)
    pub fn subtype_anchors<'a>(&'a self, store: &'a DrawingStore) -> Vec<&SubtypeAnchors> {
        let subtype_anchors = store
            .iter_subtype_anchors()
            .find(|subtype_anchors| subtype_anchors.1.isaui_id == self.id);
        match subtype_anchors {
            Some(ref subtype_anchors) => vec![subtype_anchors.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
