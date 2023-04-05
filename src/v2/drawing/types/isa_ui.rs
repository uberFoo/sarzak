// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-use-statements"}}}
use uuid::Uuid;

use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::relationship_ui::RelationshipUi;
use crate::v2::drawing::types::subtype_anchors::SubtypeAnchors;
use crate::v2::sarzak::types::isa::Isa;
use serde::{Deserialize, Serialize};

use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
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
    /// R9: [`IsaUi`] 'is drawn from' [`Anchor`]
    pub from: Uuid,
    /// R11: [`IsaUi`] 'contains additional attributes to render' [`Isa`]
    pub isa: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-implementation"}}}
impl IsaUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-new"}}}
    /// Inter a new 'IsaUI' in the store, and return it's `id`.
    pub fn new(from: &Anchor, isa: &Isa, store: &mut DrawingStore) -> IsaUi {
        let id = Uuid::new_v4();
        let new = IsaUi {
            id: id,
            from: from.id,
            isa: isa.id,
        };
        store.inter_isa_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`Anchor`] across R9(1-*)
    pub fn r9_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-forward-to-isa"}}}
    /// Navigate to [`Isa`] across R11(1-*)
    pub fn r11_isa<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Isa> {
        vec![store.exhume_isa(&self.isa).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-backward-assoc-one-cond-to-subtype_anchors"}}}
    /// Navigate to [`SubtypeAnchors`] across R10(1-1c)
    pub fn r10_subtype_anchors<'a>(&'a self, store: &'a DrawingStore) -> Vec<&SubtypeAnchors> {
        let subtype_anchors = store
            .iter_subtype_anchors()
            .find(|subtype_anchors| subtype_anchors.isaui_id == self.id);
        match subtype_anchors {
            Some(ref subtype_anchors) => vec![subtype_anchors],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-impl-nav-subtype-to-supertype-relationship_ui"}}}
    // Navigate to [`RelationshipUi`] across R6(isa)
    pub fn r6_relationship_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&RelationshipUi> {
        vec![store.exhume_relationship_ui(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
