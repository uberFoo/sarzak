// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::drawing::UUID_NS;

// Referrer imports
use crate::v2::drawing::types::edge::Edge;
use crate::v2::drawing::types::point::Point;
use crate::v2::drawing::types::subtype_anchors::SubtypeAnchors;

// Referent imports
use crate::v2::drawing::types::associative_ui::AssociativeUi;
use crate::v2::drawing::types::binary_ui::BinaryUi;
use crate::v2::drawing::types::isa_ui::IsaUi;

use crate::v2::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-documentation"}}}
/// An anchor, or anchor point, is the location where an arrow from a relationship attached
/// to an object.
///
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches, we are related
/// to an [Edge], which is related to a box, which is related to the [Object] to which we are
/// attached. This of course completes the circuit from the [Relationship] for which we are
/// drawing the lines in the first place.
///
/// Anchor also contains a direction, so that we know the orientation to draw the arrows. Finally
///, there is an offset, which is a point that describes the offset from the anchor for the
/// relationship phrase.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Anchor {
    pub id: Uuid,
    /// R3: [`Anchor`] 'has an' [`Edge`]
    pub edge: Uuid,
    /// R4: [`Anchor`] 'has a location, formalized by a' [`Point`]
    pub location: Uuid,
    /// R5: [`Anchor`] 'has a phrase offset' [`Point`]
    pub offset: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-implementation"}}}
impl Anchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-new"}}}
    /// Inter a new Anchor in the store, and return it's `id`.
    pub fn new(edge: &Edge, location: &Point, offset: &Point, store: &mut DrawingStore) -> Anchor {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}:{:?}", edge, location, offset).as_bytes(),
        );
        let new = Anchor {
            edge: edge.id(),
            location: location.id,
            offset: offset.id,
            id,
        };
        store.inter_anchor(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`Edge`] across R3(1-*)
    pub fn r3_edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Edge> {
        vec![store.exhume_edge(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-location"}}}
    /// Navigate to [`Point`] across R4(1-*)
    pub fn r4_point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        vec![store.exhume_point(&self.location).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-offset"}}}
    /// Navigate to [`Point`] across R5(1-*)
    pub fn r5_point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        vec![store.exhume_point(&self.offset).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-location"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R14(1-1c)
    pub fn r14c_associative_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        let associative_ui = store
            .iter_associative_ui()
            .find(|associative_ui| associative_ui.one == self.id);
        match associative_ui {
            Some(ref associative_ui) => vec![associative_ui],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R15(1-1c)
    pub fn r15c_associative_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        let associative_ui = store
            .iter_associative_ui()
            .find(|associative_ui| associative_ui.other == self.id);
        match associative_ui {
            Some(ref associative_ui) => vec![associative_ui],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R16(1-1c)
    pub fn r16c_associative_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        let associative_ui = store
            .iter_associative_ui()
            .find(|associative_ui| associative_ui.middle == self.id);
        match associative_ui {
            Some(ref associative_ui) => vec![associative_ui],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-binary_ui"}}}
    /// Navigate to [`BinaryUi`] across R8(1-1c)
    pub fn r8c_binary_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&BinaryUi> {
        let binary_ui = store
            .iter_binary_ui()
            .find(|binary_ui| binary_ui.to == self.id);
        match binary_ui {
            Some(ref binary_ui) => vec![binary_ui],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-binary_ui"}}}
    /// Navigate to [`BinaryUi`] across R7(1-1c)
    pub fn r7c_binary_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&BinaryUi> {
        let binary_ui = store
            .iter_binary_ui()
            .find(|binary_ui| binary_ui.from == self.id);
        match binary_ui {
            Some(ref binary_ui) => vec![binary_ui],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-isa_ui"}}}
    /// Navigate to [`IsaUi`] across R9(1-1c)
    pub fn r9c_isa_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&IsaUi> {
        let isa_ui = store.iter_isa_ui().find(|isa_ui| isa_ui.from == self.id);
        match isa_ui {
            Some(ref isa_ui) => vec![isa_ui],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-assoc_many-to-subtype_anchors"}}}
    /// Navigate to [`SubtypeAnchors`] across R10(1-M)
    pub fn r10_subtype_anchors<'a>(&'a self, store: &'a DrawingStore) -> Vec<&SubtypeAnchors> {
        store
            .iter_subtype_anchors()
            .filter_map(|subtype_anchors| {
                if subtype_anchors.anchor_id == self.id {
                    Some(subtype_anchors)
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
