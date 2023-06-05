// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"line_segment_point-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-use-statements"}}}
use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::v2::merlin::types::line_segment::LineSegment;
use crate::v2::merlin::types::point::Point;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LineSegmentPoint {
    pub id: Uuid,
    /// R5: [`LineSegment`] '🚧 Out of order — see sarzak#14.' [`LineSegment`]
    pub segment: Uuid,
    /// R5: [`Point`] '🚧 Out of order — see sarzak#14.' [`Point`]
    pub point: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-implementation"}}}
impl LineSegmentPoint {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-struct-impl-new"}}}
    /// Inter a new 'Line Segment Point' in the store, and return it's `id`.
    pub fn new(
        segment: &Arc<RwLock<LineSegment>>,
        point: &Arc<RwLock<Point>>,
        store: &mut MerlinStore,
    ) -> Arc<RwLock<LineSegmentPoint>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(LineSegmentPoint {
            id,
            segment: segment.read().unwrap().id,
            point: point.read().unwrap().id,
        }));
        store.inter_line_segment_point(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-struct-impl-nav-forward-assoc-to-point"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-struct-impl-nav-forward-assoc-to-segment"}}}
    /// Navigate to [`LineSegment`] across R5(1-*)
    pub fn r5_line_segment<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<LineSegment>>> {
        vec![store.exhume_line_segment(&self.segment).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment_point-struct-impl-nav-forward-assoc-to-point"}}}
    /// Navigate to [`Point`] across R5(1-*)
    pub fn r5_point<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<Point>>> {
        vec![store.exhume_point(&self.point).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
