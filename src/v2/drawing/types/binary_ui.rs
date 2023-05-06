// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-use-statements"}}}
use uuid::Uuid;

use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::relationship_ui::RelationshipUi;
use crate::v2::sarzak::types::binary::Binary;
use serde::{Deserialize, Serialize};

use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-documentation"}}}
/// This represents additional information necessary to render a `Binary` relationship in the
///  user interface.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BinaryUi {
    pub id: Uuid,
    /// R8: [`BinaryUi`] 'is drawn to' [`Anchor`]
    pub to: Uuid,
    /// R7: [`BinaryUi`] 'is drawn from' [`Anchor`]
    pub from: Uuid,
    /// R12: [`BinaryUi`] 'contains additional attributes to render' [`Binary`]
    pub binary_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-implementation"}}}
impl BinaryUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-new"}}}
    /// Inter a new 'BinaryUI' in the store, and return it's `id`.
    pub fn new(
        to: &Anchor,
        from: &Anchor,
        binary_id: &Binary,
        store: &mut DrawingStore,
    ) -> BinaryUi {
        let id = Uuid::new_v4();
        let new = BinaryUi {
            id,
            to: to.id,
            from: from.id,
            binary_id: binary_id.id,
        };
        store.inter_binary_ui(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-from"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-to"}}}
    /// Navigate to [`Anchor`] across R8(1-*)
    pub fn r8_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.to).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`Anchor`] across R7(1-*)
    pub fn r7_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-binary_id"}}}
    /// Navigate to [`Binary`] across R12(1-*)
    pub fn r12_binary<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Binary> {
        vec![store.exhume_binary(&self.binary_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-impl-nav-subtype-to-supertype-relationship_ui"}}}
    // Navigate to [`RelationshipUi`] across R6(isa)
    pub fn r6_relationship_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&RelationshipUi> {
        vec![store.exhume_relationship_ui(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
