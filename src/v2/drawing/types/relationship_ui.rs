// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-use-statements"}}}
use crate::v2::drawing::types::associative_ui::AssociativeUi;
use crate::v2::drawing::types::binary_ui::BinaryUi;
use crate::v2::drawing::types::isa_ui::IsaUi;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-enum-documentation"}}}
/// Additional information necessary to render relationships in the user interface.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RelationshipUi {
    AssociativeUi(Uuid),
    BinaryUi(Uuid),
    IsaUi(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-implementation"}}}
impl RelationshipUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-new-impl"}}}
    /// Create a new instance of RelationshipUi::AssociativeUi
    pub fn new_associative_ui(associative_ui: &AssociativeUi, store: &mut DrawingStore) -> Self {
        let new = Self::AssociativeUi(associative_ui.id);
        store.inter_relationship_ui(new.clone());
        new
    }

    /// Create a new instance of RelationshipUi::BinaryUi
    pub fn new_binary_ui(binary_ui: &BinaryUi, store: &mut DrawingStore) -> Self {
        let new = Self::BinaryUi(binary_ui.id);
        store.inter_relationship_ui(new.clone());
        new
    }

    /// Create a new instance of RelationshipUi::IsaUi
    pub fn new_isa_ui(isa_ui: &IsaUi, store: &mut DrawingStore) -> Self {
        let new = Self::IsaUi(isa_ui.id);
        store.inter_relationship_ui(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            RelationshipUi::AssociativeUi(id) => *id,
            RelationshipUi::BinaryUi(id) => *id,
            RelationshipUi::IsaUi(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
