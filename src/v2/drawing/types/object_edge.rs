// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::drawing::UUID_NS;

// Referrer imports
use crate::v2::drawing::types::edge::Edge;
use crate::v2::drawing::types::object_ui::ObjectUi;

use crate::v2::drawing::store::ObjectStore as DrawingStore;
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
    /// R19: [`ObjectEdge`] 'contains an' [`Edge`]
    pub edge: Uuid,
    /// R18: [`ObjectEdge`] 'comprises an' [`ObjectUi`]
    pub oui_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-implementation"}}}
impl ObjectEdge {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-new"}}}
    /// Inter a new ObjectEdge in the store, and return it's `id`.
    pub fn new(edge: &Edge, oui_id: &ObjectUi, store: &mut DrawingStore) -> ObjectEdge {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}:{:?}", edge, oui_id).as_bytes());
        let new = ObjectEdge {
            edge: edge.id(),
            oui_id: oui_id.id,
            id,
        };
        store.inter_object_edge(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`Edge`] across R19(1-*)
    pub fn r19_edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Edge> {
        vec![store.exhume_edge(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-nav-forward-to-oui_id"}}}
    /// Navigate to [`ObjectUi`] across R18(1-*)
    pub fn r18_object_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&ObjectUi> {
        vec![store.exhume_object_ui(&self.oui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
