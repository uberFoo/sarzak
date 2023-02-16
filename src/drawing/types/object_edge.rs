// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referrer imports
use crate::drawing::types::edge::Edge;
use crate::drawing::types::object_ui::ObjectUi;

use crate::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-documentation"}}}
/// The Edge of an Object Depiction
///
/// There are four edges to a rendered object.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectEdge {
    pub id: Uuid,
    /// R19: [`ObjectEdge`] 'is an aspect of' [`Edge`]
    pub edge: Option<Uuid>,
    /// R18: [`ObjectEdge`] 'is comprised of four' [`ObjectUi`]
    pub oui_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-implementation"}}}
impl ObjectEdge {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-new"}}}
    /// Inter a new ObjectEdge in the store, and return it's `id`.
    pub fn new(edge: Option<&Edge>, oui_id: &ObjectUi, store: &mut DrawingStore) -> ObjectEdge {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}:{:?}", edge, oui_id).as_bytes());
        let new = ObjectEdge {
            edge: edge.map(|edge| edge.id),
            oui_id: oui_id.id,
            id,
        };
        store.inter_object_edge(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-nav-forward-cond-to-edge"}}}
    /// Navigate to [`Edge`] across R19(1-?c)
    pub fn edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Edge> {
        match self.edge {
            Some(ref edge) => vec![store.exhume_edge(edge).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-nav-forward-to-oui_id"}}}
    /// Navigate to [`ObjectUi`] across R18(1-?)
    pub fn object_ui_r18<'a>(&'a self, store: &'a DrawingStore) -> Vec<&ObjectUi> {
        vec![store.exhume_object_ui(&self.oui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
