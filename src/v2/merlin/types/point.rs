// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"point-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-use-statements"}}}
use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::v2::merlin::types::anchor::Anchor;
use crate::v2::merlin::types::bisection::Bisection;
use crate::v2::merlin::types::inflection::INFLECTION;
use crate::v2::merlin::types::line_segment_point::LineSegmentPoint;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-hybrid-documentation"}}}
/// A two dimensional point
///
/// This is a two-tuple consisting of, say `x` and `y`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
    pub subtype: PointEnum,
    pub id: Uuid,
    pub x: i64,
    pub y: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PointEnum {
    Anchor(Uuid),
    Bisection(Uuid),
    Inflection(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-implementation"}}}
impl Point {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_anchor"}}}
    /// Inter a new Point in the store, and return it's `id`.
    pub fn new_anchor(
        x: i64,
        y: i64,
        subtype: Arc<RwLock<Anchor>>,
        store: &mut MerlinStore,
    ) -> Arc<RwLock<Point>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.read().unwrap().id;
        let new = Arc::new(RwLock::new(Point {
            x: x,
            y: y,
            subtype: PointEnum::Anchor(subtype.read().unwrap().id),
            id,
        }));
        store.inter_point(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_anchor_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_bisection"}}}
    /// Inter a new Point in the store, and return it's `id`.
    pub fn new_bisection(
        x: i64,
        y: i64,
        subtype: Arc<RwLock<Bisection>>,
        store: &mut MerlinStore,
    ) -> Arc<RwLock<Point>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.read().unwrap().id;
        let new = Arc::new(RwLock::new(Point {
            x: x,
            y: y,
            subtype: PointEnum::Bisection(subtype.read().unwrap().id),
            id,
        }));
        store.inter_point(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_bisection_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_inflection"}}}
    /// Inter a new Point in the store, and return it's `id`.
    pub fn new_inflection(x: i64, y: i64, store: &mut MerlinStore) -> Arc<RwLock<Point>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = INFLECTION;
        let new = Arc::new(RwLock::new(Point {
            x: x,
            y: y,
            subtype: PointEnum::Inflection(INFLECTION),
            id,
        }));
        store.inter_point(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-forward-to-segment"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_inflection_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-assoc-one-to-line_segment_point"}}}
    /// Navigate to [`LineSegmentPoint`] across R5(1-1)
    pub fn r5_line_segment_point<'a>(
        &'a self,
        store: &'a MerlinStore,
    ) -> Vec<Arc<RwLock<LineSegmentPoint>>> {
        vec![store
            .iter_line_segment_point()
            .find(|line_segment_point| line_segment_point.read().unwrap().point == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
