// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::edge::Edge;
use crate::v2::merlin::types::glyph::Glyph;
use crate::v2::merlin::types::line::Line;
use crate::v2::merlin::types::point::Point;
use crate::v2::merlin::types::relationship_phrase::RelationshipPhrase;
use crate::v2::merlin::types::x_box::XBox;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
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
    /// R3: [`XBox`] '🚧 Out of order — see sarzak#14.' [`XBox`]
    pub x_box: Uuid,
    /// R3: [`Line`] '🚧 Out of order — see sarzak#14.' [`Line`]
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
        edge: &Edge,
        glyph: &Glyph,
        x_box: &XBox,
        line: &Line,
        store: &mut MerlinStore,
    ) -> Anchor {
        let id = Uuid::new_v4();
        let new = Anchor {
            id: id,
            offset: offset,
            x_offset: x_offset,
            y_offset: y_offset,
            edge: edge.id(),
            glyph: glyph.id,
            x_box: x_box.id,
            line: line.id,
        };
        store.inter_anchor(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`Edge`] across R9(1-*)
    pub fn r9_edge<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Edge> {
        vec![store.exhume_edge(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-to-glyph"}}}
    /// Navigate to [`Glyph`] across R10(1-*)
    pub fn r10_glyph<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Glyph> {
        vec![store.exhume_glyph(&self.glyph).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-cond-to-relationship_phrase"}}}
    /// Navigate to [`RelationshipPhrase`] across R13(1-1c)
    pub fn r13c_relationship_phrase<'a>(
        &'a self,
        store: &'a MerlinStore,
    ) -> Vec<&RelationshipPhrase> {
        let relationship_phrase = store
            .iter_relationship_phrase()
            .find(|relationship_phrase| relationship_phrase.origin == self.id);
        match relationship_phrase {
            Some(ref relationship_phrase) => vec![relationship_phrase],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-assoc-to-line"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-assoc-to-x_box"}}}
    /// Navigate to [`XBox`] across R3(1-*)
    pub fn r3_x_box<'a>(&'a self, store: &'a MerlinStore) -> Vec<&XBox> {
        vec![store.exhume_x_box(&self.x_box).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-forward-assoc-to-line"}}}
    /// Navigate to [`Line`] across R3(1-*)
    pub fn r3_line<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Line> {
        vec![store.exhume_line(&self.line).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-impl-nav-subtype-to-supertype-point"}}}
    // Navigate to [`Point`] across R6(isa)
    pub fn r6_point<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Point> {
        vec![store.exhume_point(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
