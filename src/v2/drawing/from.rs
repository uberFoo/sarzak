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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-from-impl-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-from-impl-definition"}}}
use crate::v2::drawing::types::{
    Anchor, AssociativeUi, BinaryUi, IsaUi, ObjectEdge, ObjectUi, Point, SubtypeAnchors,
};
use crate::v2::drawing::ObjectStore;

use crate::v1::drawing::types::{
    Anchor as FromAnchor, AssociativeUi as FromAssociativeUi, BinaryUi as FromBinaryUi,
    IsaUi as FromIsaUi, ObjectEdge as FromObjectEdge, ObjectUi as FromObjectUi, Point as FromPoint,
    SubtypeAnchors as FromSubtypeAnchors,
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
            edge: src.edge,
            location: src.location,
            offset: src.offset,
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

impl From<&FromSubtypeAnchors> for SubtypeAnchors {
    fn from(src: &FromSubtypeAnchors) -> Self {
        Self {
            id: src.id,
            isaui_id: src.isaui_id,
            anchor_id: src.anchor_id,
        }
    }
}

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
