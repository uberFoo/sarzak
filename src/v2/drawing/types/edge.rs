// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-use-statements"}}}
use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::bottom::BOTTOM;
use crate::v2::drawing::types::left::LEFT;
use crate::v2::drawing::types::object_edge::ObjectEdge;
use crate::v2::drawing::types::right::RIGHT;
use crate::v2::drawing::types::top::TOP;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-enum-documentation"}}}
/// An attachment point for an [Anchor]
///
/// It’s used with [Anchor] to orient the arrows on the ends of the lines according to the
///  side of the box to which they are attached. Some arrows are on top, some bottom, etc.
///
/// This is not rendered as a visible item. The [ObjectUI] manages that by itself. This instead
///  renders an invisible line. The line is used for several things. For one, when hovered over
///  the cursor changes to the appropriate one for resizing.
///
/// Also, this is used to register where relationship may anchor.
///
/// It’s this last regard that is somewhat concerning. Indicating that an anchor is attached
///  to an edge get’s us the connection we need between an [Object] and a [Relationship]. But
///  it’s under-specified. It doesn’t indicate where along the edge the arrow is connected
/// .
///
/// I’m considering put a relationship back between [Anchor] and [Point].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn new_bottom(store: &DrawingStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_edge(&BOTTOM).unwrap()
    }

    /// Create a new instance of Edge::Left
    pub fn new_left(store: &DrawingStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_edge(&LEFT).unwrap()
    }

    /// Create a new instance of Edge::Right
    pub fn new_right(store: &DrawingStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_edge(&RIGHT).unwrap()
    }

    /// Create a new instance of Edge::Top
    pub fn new_top(store: &DrawingStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_edge(&TOP).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Bottom(id) => *id,
            Self::Left(id) => *id,
            Self::Right(id) => *id,
            Self::Top(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-struct-impl-nav-backward-cond-to-anchor"}}}
    /// Navigate to [`Anchor`] across R3(1-1c)
    pub fn r3c_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<Arc<RwLock<Anchor>>> {
        span!("r3_anchor");
        let anchor = store
            .iter_anchor()
            .find(|anchor| anchor.read().unwrap().edge == self.id());
        match anchor {
            Some(ref anchor) => vec![anchor.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-struct-impl-nav-backward-cond-to-object_edge"}}}
    /// Navigate to [`ObjectEdge`] across R19(1-1c)
    pub fn r19c_object_edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<Arc<RwLock<ObjectEdge>>> {
        span!("r19_object_edge");
        let object_edge = store
            .iter_object_edge()
            .find(|object_edge| object_edge.read().unwrap().edge == self.id());
        match object_edge {
            Some(ref object_edge) => vec![object_edge.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
