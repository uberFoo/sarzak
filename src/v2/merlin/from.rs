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
use std::sync::{Arc, RwLock};

// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"v2::drawing-from-impl-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"v2::drawing-from-impl-definition"}}}
use crate::v2::merlin::types::{
    Anchor, Bisection, Edge, Glyph, Line, LineSegment, LineSegmentPoint, Point, RelationshipName,
    XBox, BOTTOM, LEFT, RIGHT, TOP,
};
use crate::v2::merlin::ObjectStore;

use crate::v2::drawing::types::{Edge as FromEdge, ObjectUi, Point as FromPoint};
use crate::v2::drawing::ObjectStore as DrawingStore;

use crate::v2::sarzak::types::Cardinality;
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
/// Well darn. There isn't a good way to get an object from an anchor, nor an
/// anchor from an object -- not in the drawing domain anyway.
///
impl From<(&DrawingStore, &SarzakStore)> for ObjectStore {
    fn from((drawing, sarzak): (&DrawingStore, &SarzakStore)) -> Self {
        let mut merlin = ObjectStore::new();

        merlin.inter_edge(Arc::new(RwLock::new(Edge::Bottom(BOTTOM))));
        merlin.inter_edge(Arc::new(RwLock::new(Edge::Left(LEFT))));
        merlin.inter_edge(Arc::new(RwLock::new(Edge::Right(RIGHT))));
        merlin.inter_edge(Arc::new(RwLock::new(Edge::Top(TOP))));

        for oui in drawing.iter_object_ui() {
            let instance = XBox::from((oui, drawing));
            merlin.inter_x_box(Arc::new(RwLock::new(instance)));
        }

        for bui in drawing.iter_binary_ui() {
            let binary = bui.r12_binary(sarzak)[0];
            let rel = binary.r4_relationship(sarzak)[0];

            let line = Line::new(&rel, &mut merlin);
            let line_seg = LineSegment::new(line.clone(), &mut merlin);

            // Default to putting the relationship at the midpoint of the line,
            let bisection = Bisection::new(0.5, line_seg.clone(), &mut merlin);
            let name = RelationshipName::new(
                format!("R{}", binary.number),
                0,
                0,
                bisection,
                line.clone(),
                &mut merlin,
            );

            let from_anchor = bui.r7_anchor(drawing)[0];

            // Our relationships are jacked up, so we have to do the dumb thing.
            // Not that relationship navigation is any better, depending on the
            // direction.
            let referrer = binary.r6_referrer(sarzak)[0];
            let from_obj = referrer.r17_object(sarzak)[0];
            let from_obj_ui = drawing
                .iter_object_ui()
                .find(|oui| oui.object_id == from_obj.id)
                .unwrap();

            // Get what we need to build the offset to which the line connects.
            let point = from_anchor.r4_point(drawing)[0];
            let edge = from_anchor.r3_edge(drawing)[0];
            let (x, y) = get_anchor_offset(&point, &edge);

            // Sort out how far along the edge the arrow should be drawn.
            let origin = from_obj_ui.r13_point(drawing)[0];
            let offset = get_anchor_line_offset(&edge, &point, &from_obj_ui, &origin);

            // Sort out the glyph.
            let card = referrer.r9_cardinality(sarzak)[0];
            let glyph = match card {
                Cardinality::One(_) => Glyph::new_one(line.clone(), &mut merlin),
                Cardinality::Many(_) => Glyph::new_many(line.clone(), &mut merlin),
            };

            // Get the box.
            let x_box = merlin.exhume_x_box(&from_obj_ui.id).unwrap().clone();

            // Create the anchor.
            let from_anchor = Anchor::new(
                offset,
                x,
                y,
                XyzzyEdge(edge, &merlin).into(),
                glyph,
                x_box,
                line.clone(),
                &mut merlin,
            );

            // Create the from point
            let point = Point::new_anchor(point.x, point.y, from_anchor, &mut merlin);

            // Create the "line segment point"
            LineSegmentPoint::new(line_seg.clone(), point, &mut merlin);

            let to_anchor = bui.r8_anchor(drawing)[0];
            let referent = binary.r5_referent(sarzak)[0];
            let from_obj = referent.r16_object(sarzak)[0];
            let from_obj_ui = drawing
                .iter_object_ui()
                .find(|oui| oui.object_id == from_obj.id)
                .unwrap();

            // Get what we need to build the offset to which the line connects.
            let point = to_anchor.r4_point(drawing)[0];
            let edge = to_anchor.r3_edge(drawing)[0];
            let (x, y) = get_anchor_offset(&point, &edge);

            // Sort out how far along the edge the arrow should be drawn.
            let origin = from_obj_ui.r13_point(drawing)[0];
            let offset = get_anchor_line_offset(&edge, &point, &from_obj_ui, &origin);

            // Sort out the glyph.
            let card = referent.r8_cardinality(sarzak)[0];
            let glyph = match card {
                Cardinality::One(_) => Glyph::new_one(line.clone(), &mut merlin),
                Cardinality::Many(_) => Glyph::new_many(line.clone(), &mut merlin),
            };

            // Get the box.
            let x_box = merlin.exhume_x_box(&from_obj_ui.id).unwrap().clone();

            // Create the anchor.
            let to_anchor = Anchor::new(
                offset,
                x,
                y,
                XyzzyEdge(edge, &merlin).into(),
                glyph,
                x_box,
                line.clone(),
                &mut merlin,
            );

            // Create the to point
            let point = Point::new_anchor(point.x, point.y, to_anchor, &mut merlin);

            // Create the "line segment point"
            LineSegmentPoint::new(line_seg, point, &mut merlin);
        }

        merlin
    }
}

fn get_anchor_offset(point: &FromPoint, edge: &FromEdge) -> (i64, i64) {
    let (x, y) = (point.x, point.y);

    match edge {
        FromEdge::Top(_) => (x, y - 40),
        FromEdge::Right(_) => (x + 40, y),
        FromEdge::Bottom(_) => (x, y + 40),
        FromEdge::Left(_) => (x - 40, y),
    }
}

fn get_anchor_line_offset(
    edge: &FromEdge,
    anchor: &FromPoint,
    obj: &ObjectUi,
    origin: &FromPoint,
) -> f64 {
    let (x, y) = (anchor.x, anchor.y);
    let (obj_x, obj_y) = (origin.x, origin.y);

    match edge {
        FromEdge::Top(_) | FromEdge::Bottom(_) => obj.width as f64 / (x + obj.width - obj_x) as f64,
        FromEdge::Left(_) | FromEdge::Right(_) => {
            obj.height as f64 / (y + obj.height - obj_y) as f64
        }
    }
}

impl From<(&ObjectUi, &DrawingStore)> for XBox {
    fn from((src, store): (&ObjectUi, &DrawingStore)) -> Self {
        let point = src.r13_point(store)[0];
        Self {
            id: src.id,
            x: point.x,
            y: point.y,
            width: src.width,
            height: src.height,
            object: src.object_id,
        }
    }
}

struct XyzzyEdge<'a>(&'a FromEdge, &'a ObjectStore);

impl<'a> From<XyzzyEdge<'a>> for Arc<RwLock<Edge>> {
    fn from(edge: XyzzyEdge<'a>) -> Self {
        let src = edge.0;
        let merlin = edge.1;

        match src {
            FromEdge::Bottom(_) => merlin.exhume_edge(&BOTTOM).unwrap(),
            FromEdge::Left(_) => merlin.exhume_edge(&LEFT).unwrap(),
            FromEdge::Right(_) => merlin.exhume_edge(&RIGHT).unwrap(),
            FromEdge::Top(_) => merlin.exhume_edge(&TOP).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::DomainBuilder;

    #[test]
    fn test_from_drawing() {
        let _ = env_logger::builder().is_test(true).try_init();

        let drawing = DomainBuilder::new()
            .cuckoo_model("models/drawing.json")
            .unwrap()
            .build_v2()
            .unwrap();

        dbg!(&drawing.merlin());
    }
}

// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
