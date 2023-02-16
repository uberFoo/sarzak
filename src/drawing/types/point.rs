// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"point-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referent imports
use crate::drawing::types::anchor::Anchor;
use crate::drawing::types::associative_ui::AssociativeUi;
use crate::drawing::types::object_ui::ObjectUi;

use crate::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-documentation"}}}
/// A point is a two-tuple that represents a location on the drawing canvas.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
    pub id: Uuid,
    pub x: i64,
    pub y: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-implementation"}}}
impl Point {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new"}}}
    /// Inter a new Point in the store, and return it's `id`.
    pub fn new(x: i64, y: i64, store: &mut DrawingStore) -> Point {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{}", x, y).as_bytes());
        let new = Point { x: x, y: y, id };
        store.inter_point(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-one-to-anchor"}}}
    /// Navigate to [`Anchor`] across R4(1-1)
    pub fn anchor_r4<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![
            store
                .iter_anchor()
                .find(|anchor| anchor.1.location == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-one-to-anchor"}}}
    /// Navigate to [`Anchor`] across R5(1-1)
    pub fn anchor_r5<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![
            store
                .iter_anchor()
                .find(|anchor| anchor.1.offset == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-one-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R17(1-1)
    pub fn associative_ui_r17<'a>(&'a self, store: &'a DrawingStore) -> Vec<&AssociativeUi> {
        vec![
            store
                .iter_associative_ui()
                .find(|associative_ui| associative_ui.1.from == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-one-to-object_ui"}}}
    /// Navigate to [`ObjectUi`] across R13(1-1)
    pub fn object_ui_r13<'a>(&'a self, store: &'a DrawingStore) -> Vec<&ObjectUi> {
        vec![
            store
                .iter_object_ui()
                .find(|object_ui| object_ui.1.origin == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
