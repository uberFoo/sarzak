//! v2::drawing Object From Trait Implementations
//!
//! These are [`From`] trait implementations for the domain: _drawing_. They are
//! generated to be used during the extrusion process. This is the process
//! by which instances of one domain are transformed into instances of another.
//! In this case the source domain is `v1::drawing`.
//!
//! It is hoped that the model has not changed enough to render
//! these implementations useless. In any case it's expected that
//! the generated code will need to be manually edited.
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"v2::drawing-from-impl-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"v2::drawing-from-impl-definition"}}}
use uuid::Uuid;

use crate::v2::drawing::types::{
    Anchor, AssociativeUi, BinaryUi, Edge, IsaUi, ObjectEdge, ObjectUi, Point, RelationshipUi,
    SubtypeAnchors, BOTTOM, LEFT, RIGHT, TOP,
};
use crate::v2::drawing::ObjectStore;

use crate::v1::drawing::types::{
    Anchor as FromAnchor, AssociativeUi as FromAssociativeUi, BinaryUi as FromBinaryUi,
    Edge as FromEdge, IsaUi as FromIsaUi, ObjectEdge as FromObjectEdge, ObjectUi as FromObjectUi,
    Point as FromPoint, RelationshipUi as FromRelationshipUi, SubtypeAnchors as FromSubtypeAnchors,
    BOTTOM as FROM_BOTTOM, LEFT as FROM_LEFT, RIGHT as FROM_RIGHT, TOP as FROM_TOP,
};
use crate::v1::drawing::ObjectStore as DrawingStore;

impl From<&DrawingStore> for ObjectStore {
    fn from(from: &DrawingStore) -> Self {
        let mut to = ObjectStore::new();

        for (_, instance) in from.iter_anchor() {
            let instance = Anchor::from(instance);
            to.inter_anchor(instance);
        }

        for (_, instance) in from.iter_associative_ui() {
            let instance = AssociativeUi::from(instance);
            to.inter_associative_ui(instance);
        }

        for (_, instance) in from.iter_binary_ui() {
            let instance = BinaryUi::from(instance);
            to.inter_binary_ui(instance);
        }

        for (_, instance) in from.iter_edge() {
            let instance = Edge::from(instance);
            to.inter_edge(instance);
        }

        for (_, instance) in from.iter_isa_ui() {
            let instance = IsaUi::from(instance);
            to.inter_isa_ui(instance);
        }

        for (_, instance) in from.iter_object_edge() {
            let instance = ObjectEdge::from(instance);
            to.inter_object_edge(instance);
        }

        for (_, instance) in from.iter_object_ui() {
            let instance = ObjectUi::from(instance);
            to.inter_object_ui(instance);
        }

        for (_, instance) in from.iter_point() {
            let instance = Point::from(instance);
            to.inter_point(instance);
        }

        for (_, instance) in from.iter_relationship_ui() {
            let instance = RelationshipUi::from(instance);
            to.inter_relationship_ui(instance);
        }

        for (_, instance) in from.iter_subtype_anchors() {
            let instance = SubtypeAnchors::from(instance);
            to.inter_subtype_anchors(instance);
        }

        to
    }
}

impl From<&FromAnchor> for Anchor {
    fn from(src: &FromAnchor) -> Self {
        Self {
            id: src.id,
            edge: swizzle_edge(src.edge),
            offset: src.offset,
            location: src.location,
        }
    }
}

impl From<&FromAssociativeUi> for AssociativeUi {
    fn from(src: &FromAssociativeUi) -> Self {
        Self {
            id: src.id,
            other: src.other,
            middle: src.middle,
            one: src.one,
            associative_id: src.associative_id,
            from: src.from,
        }
    }
}

impl From<&FromBinaryUi> for BinaryUi {
    fn from(src: &FromBinaryUi) -> Self {
        Self {
            id: src.id,
            from: src.from,
            to: src.to,
            binary_id: src.binary_id,
        }
    }
}

impl From<&FromEdge> for Edge {
    fn from(src: &FromEdge) -> Self {
        match src {
            FromEdge::Bottom(_) => Edge::Bottom(BOTTOM),
            FromEdge::Left(_) => Edge::Left(LEFT),
            FromEdge::Right(_) => Edge::Right(RIGHT),
            FromEdge::Top(_) => Edge::Top(TOP),
        }
    }
}

fn swizzle_edge(src: Uuid) -> Uuid {
    match src {
        FROM_BOTTOM => BOTTOM,
        FROM_LEFT => LEFT,
        FROM_RIGHT => RIGHT,
        FROM_TOP => TOP,
        _ => panic!("Unexpected edge: {:?}", src),
    }
}

impl From<&FromIsaUi> for IsaUi {
    fn from(src: &FromIsaUi) -> Self {
        Self {
            id: src.id,
            from: src.from,
            isa: src.isa,
        }
    }
}

impl From<&FromObjectEdge> for ObjectEdge {
    fn from(src: &FromObjectEdge) -> Self {
        Self {
            id: src.id,
            edge: src.edge,
            oui_id: src.oui_id,
        }
    }
}

impl From<&FromObjectUi> for ObjectUi {
    fn from(src: &FromObjectUi) -> Self {
        Self {
            height: src.height,
            id: src.id,
            width: src.width,
            object_id: src.object_id,
            origin: src.origin,
        }
    }
}

impl From<&FromPoint> for Point {
    fn from(src: &FromPoint) -> Self {
        Self {
            id: src.id,
            x: src.x,
            y: src.y,
        }
    }
}

impl From<&FromRelationshipUi> for RelationshipUi {
    fn from(src: &FromRelationshipUi) -> Self {
        match src {
            FromRelationshipUi::AssociativeUi(src) => RelationshipUi::AssociativeUi(src.clone()),
            FromRelationshipUi::BinaryUi(src) => RelationshipUi::BinaryUi(src.clone()),
            FromRelationshipUi::IsaUi(src) => RelationshipUi::IsaUi(src.clone()),
        }
    }
}
impl From<&FromSubtypeAnchors> for SubtypeAnchors {
    fn from(src: &FromSubtypeAnchors) -> Self {
        Self {
            id: src.id,
            isaui_id: src.isaui_id,
            anchor_id: src.anchor_id,
        }
    }
}

// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
