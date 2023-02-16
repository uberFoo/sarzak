// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

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
