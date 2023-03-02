// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-use-statements"}}}
use crate::v2::drawing::types::bottom::BOTTOM;
use crate::v2::drawing::types::left::LEFT;
use crate::v2::drawing::types::right::RIGHT;
use crate::v2::drawing::types::top::TOP;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-new-impl"}}}
    /// Create a new instance of Edge::Bottom
    pub fn new_bottom() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Bottom(BOTTOM)
    }

    /// Create a new instance of Edge::Left
    pub fn new_left() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Left(LEFT)
    }

    /// Create a new instance of Edge::Right
    pub fn new_right() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Right(RIGHT)
    }

    /// Create a new instance of Edge::Top
    pub fn new_top() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Top(TOP)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
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
