// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

// Referent imports
use crate::drawing::types::anchor::Anchor;
use crate::drawing::types::object_edge::ObjectEdge;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-enum-documentation"}}}
/// An attachment point for an [Anchor]
///
/// It’s used with [Anchor] to orient the arrows on the ends of the lines according to the
/// side of the box to which they are attached. Some arrows are on top, some bottom, etc.
///
/// This is not rendered as a visible item. The [ObjectUI] manages that by itself. This instead
/// renders an invisible line. The line is used for several things. For one, when hovered over
/// the cursor changes to the appropriate one for resizing.
///
/// Also, this is used to register where relationship may anchor.
///
/// It’s this last regard that is somewhat concerning. Indicating that an anchor is attached
/// to an edge get’s us the connection we need between an [Object] and a [Relationship]. But
/// it’s under-specified. It doesn’t indicate where along the edge the arrow is connected
///.
///
/// I’m considering put a relationship back between [Anchor] and [Point].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Edge {
    Bottom(Uuid),
    Left(Uuid),
    Right(Uuid),
    Top(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-implementation"}}}
impl Edge {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Edge::Bottom(id) => *id,
            Edge::Left(id) => *id,
            Edge::Right(id) => *id,
            Edge::Top(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
