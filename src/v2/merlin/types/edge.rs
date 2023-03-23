// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-use-statements"}}}
use crate::v2::merlin::store::ObjectStore as MerlinStore;
use crate::v2::merlin::types::anchor::Anchor;
use crate::v2::merlin::types::bottom::BOTTOM;
use crate::v2::merlin::types::left::LEFT;
use crate::v2::merlin::types::right::RIGHT;
use crate::v2::merlin::types::top::TOP;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"edge-struct-impl-nav-backward-1_M-to-anchor"}}}
    /// Navigate to [`Anchor`] across R9(1-M)
    pub fn r9_anchor<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Anchor> {
        store
            .iter_anchor()
            .filter_map(|anchor| {
                if anchor.edge == self.id() {
                    Some(anchor)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
