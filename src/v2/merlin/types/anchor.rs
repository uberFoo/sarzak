// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::merlin::types::edge::Edge;
use crate::v2::merlin::types::glyph::Glyph;
use crate::v2::merlin::types::line::Line;
use crate::v2::merlin::types::point::Point;
use crate::v2::merlin::types::point::PointEnum;
use crate::v2::merlin::types::relationship_phrase::RelationshipPhrase;
use crate::v2::merlin::types::x_box::XBox;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-documentation"}}}
/// An Anchor Point for Lines
///
/// This represents a point on the periphery of a box to which a relationship attaches.
///
/// It's really sort of clever. Once you figure out which edge, you use the `offset` attribute
///  (a float between 0.0 and 1.0) to calculate how far along that line to draw the line.
///
/// 🚧 The offsets are meant to be for the relationship phrase maybe? Drat, I'll have to figure
///  that out.🚧
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Anchor {
    pub id: Uuid,
    pub offset: f64,
    pub x_offset: i64,
    pub y_offset: i64,
    /// R9: [`Anchor`] 'connects to an' [`Edge`]
    pub edge: Uuid,
    /// R10: [`Anchor`] 'is displayed as a' [`Glyph`]
    pub glyph: Uuid,
    /// R3: [`XBox`] '🚧 Comments are out of order — see sarzak#14.' [`XBox`]
    pub x_box: Uuid,
    /// R3: [`Line`] '🚧 Comments are out of order — see sarzak#14.' [`Line`]
    pub line: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-implementation"}}}
impl Anchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-new"}}}
    /// Inter a new 'Anchor' in the store, and return it's `id`.
    pub fn new(
        offset: f64,
        x_offset: i64,
        y_offset: i64,
        edge: &Arc<RwLock<Edge>>,
        glyph: &Arc<RwLock<Glyph>>,
        x_box: &Arc<RwLock<XBox>>,
        line: &Arc<RwLock<Line>>,
        store: &mut MerlinStore,
    ) -> Arc<RwLock<Anchor>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Anchor {
            id,
            offset,
            x_offset,
            y_offset,
            edge: edge.read().unwrap().id(),
            glyph: glyph.read().unwrap().id,
            x_box: x_box.read().unwrap().id,
            line: line.read().unwrap().id,
        }));
        store.inter_anchor(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`Edge`] across R9(1-*)
    pub fn r9_edge<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<Edge>>> {
        vec![store.exhume_edge(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-glyph"}}}
    /// Navigate to [`Glyph`] across R10(1-*)
    pub fn r10_glyph<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<Glyph>>> {
        vec![store.exhume_glyph(&self.glyph).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-relationship_phrase"}}}
    /// Navigate to [`RelationshipPhrase`] across R13(1-1c)
    pub fn r13c_relationship_phrase<'a>(
        &'a self,
        store: &'a MerlinStore,
    ) -> Vec<Arc<RwLock<RelationshipPhrase>>> {
        let relationship_phrase = store
            .iter_relationship_phrase()
            .find(|relationship_phrase| relationship_phrase.read().unwrap().origin == self.id);
        match relationship_phrase {
            Some(ref relationship_phrase) => vec![relationship_phrase.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-assoc-to-line"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-assoc-to-x_box"}}}
    /// Navigate to [`XBox`] across R3(1-*)
    pub fn r3_x_box<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<XBox>>> {
        vec![store.exhume_x_box(&self.x_box).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-assoc-to-line"}}}
    /// Navigate to [`Line`] across R3(1-*)
    pub fn r3_line<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<Line>>> {
        vec![store.exhume_line(&self.line).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-impl-nav-subtype-to-supertype-point"}}}
    // Navigate to [`Point`] across R6(isa)
    pub fn r6_point<'a>(&'a self, store: &'a MerlinStore) -> Vec<Arc<RwLock<Point>>> {
        vec![store
            .iter_point()
            .find(|point| {
                if let PointEnum::Anchor(id) = point.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
