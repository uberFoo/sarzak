// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referrer imports
use crate::drawing::types::edge::Edge;
use crate::drawing::types::point::Point;
use crate::drawing::types::subtype_anchors::SubtypeAnchors;

// Referent imports
use crate::drawing::types::associative_ui::AssociativeUi;
use crate::drawing::types::binary_ui::BinaryUi;
use crate::drawing::types::isa_ui::IsaUi;

use crate::drawing::store::ObjectStore as DrawingStore;
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
    /// R3: [`Anchor`] 'describes the orientation of an' [`Edge`]
    pub edge: Option<Uuid>,
    /// R4: [`Anchor`] 'describes the location of an' [`Point`]
    pub location: Option<Uuid>,
    /// R5: [`Anchor`] 'describes an offset for an' [`Point`]
    pub offset: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-implementation"}}}
impl Anchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-new"}}}
    /// Inter a new Anchor in the store, and return it's `id`.
    pub fn new(
        edge: Option<&Edge>,
        location: Option<&Point>,
        offset: Option<&Point>,
        store: &mut DrawingStore,
    ) -> Anchor {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}:{:?}", edge, location, offset).as_bytes(),
        );
        let new = Anchor {
            edge: edge.map(|edge| edge.id),
            location: location.map(|point| point.id),
            offset: offset.map(|point| point.id),
            id,
        };
        store.inter_anchor(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-cond-to-edge"}}}
    /// Navigate to [`Edge`] across R3(1-?c)
    pub fn edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Edge> {
        match self.edge {
            Some(ref edge) => vec![store.exhume_edge(edge).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-cond-to-offset"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-cond-to-location"}}}
    /// Navigate to [`Point`] across R4(1-?c)
    pub fn point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        match self.location {
            Some(ref location) => vec![store.exhume_point(location).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-cond-to-location"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-cond-to-offset"}}}
    /// Navigate to [`Point`] across R5(1-?c)
    pub fn point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        match self.offset {
            Some(ref offset) => vec![store.exhume_point(offset).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-one-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R15(1-1)
    pub fn associative_ui_r15<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        vec![
            store
                .iter_associative_ui()
                .find(|associative_ui| associative_ui.1.other == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-one-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R16(1-1)
    pub fn associative_ui_r16<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        vec![
            store
                .iter_associative_ui()
                .find(|associative_ui| associative_ui.1.middle == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-one-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R14(1-1)
    pub fn associative_ui_r14<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        vec![
            store
                .iter_associative_ui()
                .find(|associative_ui| associative_ui.1.one == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-one-to-binary_ui"}}}
    /// Navigate to [`BinaryUi`] across R7(1-1)
    pub fn binary_ui_r7<'a>(&'a self, store: &'a DrawingStore) -> Vec<&BinaryUi> {
        vec![
            store
                .iter_binary_ui()
                .find(|binary_ui| binary_ui.1.from == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-one-to-binary_ui"}}}
    /// Navigate to [`BinaryUi`] across R8(1-1)
    pub fn binary_ui_r8<'a>(&'a self, store: &'a DrawingStore) -> Vec<&BinaryUi> {
        vec![
            store
                .iter_binary_ui()
                .find(|binary_ui| binary_ui.1.to == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-one-to-isa_ui"}}}
    /// Navigate to [`IsaUi`] across R9(1-1)
    pub fn isa_ui_r9<'a>(&'a self, store: &'a DrawingStore) -> Vec<&IsaUi> {
        vec![
            store
                .iter_isa_ui()
                .find(|isa_ui| isa_ui.1.from == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-assoc_many-to-subtype_anchors"}}}
    /// Navigate to [`SubtypeAnchors`] across R10(1-M)
    pub fn subtype_anchors<'a>(&'a self, store: &'a DrawingStore) -> Vec<&SubtypeAnchors> {
        store
            .iter_subtype_anchors()
            .filter_map(|subtype_anchors| {
                if subtype_anchors.1.anchor_id == self.id {
                    Some(subtype_anchors.1)
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
