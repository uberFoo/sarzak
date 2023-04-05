// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship_phrase-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_phrase-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::anchor::Anchor;
use crate::v2::merlin::types::line::Line;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_phrase-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RelationshipPhrase {
    pub id: Uuid,
    pub text: String,
    pub x: i64,
    pub y: i64,
    /// R13: [`RelationshipPhrase`] 'is attached to an' [`Anchor`]
    pub origin: Uuid,
    /// R12: [`RelationshipPhrase`] 'describes' [`Line`]
    pub line: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_phrase-implementation"}}}
impl RelationshipPhrase {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_phrase-struct-impl-new"}}}
    /// Inter a new 'Relationship Phrase' in the store, and return it's `id`.
    pub fn new(
        text: String,
        x: i64,
        y: i64,
        origin: &Anchor,
        line: &Line,
        store: &mut MerlinStore,
    ) -> RelationshipPhrase {
        let id = Uuid::new_v4();
        let new = RelationshipPhrase {
            id: id,
            text: text,
            x: x,
            y: y,
            origin: origin.id,
            line: line.id,
        };
        store.inter_relationship_phrase(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_phrase-struct-impl-nav-forward-to-origin"}}}
    /// Navigate to [`Anchor`] across R13(1-*)
    pub fn r13_anchor<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.origin).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_phrase-struct-impl-nav-forward-to-line"}}}
    /// Navigate to [`Line`] across R12(1-*)
    pub fn r12_line<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Line> {
        vec![store.exhume_line(&self.line).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
