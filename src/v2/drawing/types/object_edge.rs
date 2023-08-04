// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_edge-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::drawing::types::edge::Edge;
use crate::v2::drawing::types::object_ui::ObjectUi;
use serde::{Deserialize, Serialize};

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
    /// Inter a new 'Object Edge' in the store, and return it's `id`.
    pub fn new(
        edge: &Rc<RefCell<Edge>>,
        oui_id: &Rc<RefCell<ObjectUi>>,
        store: &mut DrawingStore,
    ) -> Rc<RefCell<ObjectEdge>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ObjectEdge {
            id,
            edge: edge.borrow().id(),
            oui_id: oui_id.borrow().id,
        }));
        store.inter_object_edge(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`Edge`] across R19(1-*)
    pub fn r19_edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<Edge>>> {
        span!("r19_edge");
        vec![store.exhume_edge(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_edge-struct-impl-nav-forward-to-oui_id"}}}
    /// Navigate to [`ObjectUi`] across R18(1-*)
    pub fn r18_object_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<ObjectUi>>> {
        span!("r18_object_ui");
        vec![store.exhume_object_ui(&self.oui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
