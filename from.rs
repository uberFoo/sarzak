//! v2::drawing Object From Trait Implementations
//!
//! These are [`From`] trait implementations for the domain: _Drawing_. They are
//! generated to be used during the extrusion process. This is the process
//! by which instances of one domain are transformed into instances of another.
//! In this case the source domain is `v1::drawing`.
//!
//! It is hoped that the model has not changed enough to render
//! these implementations useless. In any case it's expected that
//! the generated code will need to be manually edited.
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"v2::drawing-from-impl-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"v2::drawing-from-impl-definition"}}}
use crate::v2::drawing::types::{
    Anchor, Bisection, Edge, Glyph, Line, LineSegment, Point, RelationshipName, RelationshipPhrase,
    XBox, BOTTOM, LEFT, RIGHT, TOP,
};
use crate::v2::drawing::ObjectStore;

use crate::v1::drawing::types::{Anchor as FromAnchor, Edge as FromEdge, Point as FromPoint};
use crate::v1::drawing::ObjectStore as DrawingStore;

use crate::v2::sarzak::types::Object as FromObject;
use crate::v2::sarzak::ObjectStore as SarzakStore;

/// Convert a v1 Drawing format into a v2 Drawing format
///
/// These domains are completely different, and there is some real work taking place
/// here. We can cover the boxen separately from the lines, as per normal.
///
/// For the boxen, we can iterate over the ObjectUI from the other domain and suck
/// the details out into an instance of Box.
///
/// Relationships are much trickier. We can start at RelationshipUI and handle each
/// of the three cases separately, as they will each require different aspects of
/// the new model. This is where all of the model complication lies. Actually, much of
/// complexity is for future use. Especially the line segments, which will allow for
/// polyline relationships.
///
/// Starting with the Binary, we will have two anchors, each with a point, and edge
/// and an offset. The x and y offsets of the new anchor can be computed based on
/// these. The offset attribute is calculated, and requires the object to do so.
///
impl From<(&DrawingStore, &SarzakStore)> for ObjectStore {
    fn from((from_drawing, from_sarzak): (&DrawingStore, &SarzakStore)) -> Self {
        let mut to = ObjectStore::new();

        for instance in from_drawing.iter_anchor() {
            let instance = Anchor::from(instance.1);
            to.inter_anchor(instance);
        }

        for instance in from_drawing.iter_edge() {
            let instance = Edge::from(instance.1);
            to.inter_edge(instance);
        }

        to
    }
}

impl From<(&FromAnchor, &FromObject)> for Anchor {
    fn from(src: &FromAnchor) -> Self {
        Self {
            id: src.id,
            offset: src.offset,
            x_offset: src.x_offset,
            y_offset: src.y_offset,
            edge: src.edge,
            glyph: src.glyph,
            line: src.line,
            x_box: src.x_box,
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
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
