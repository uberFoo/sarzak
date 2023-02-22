// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::drawing::UUID_NS;

// Referrer imports
use crate::v2::drawing::types::point::Point;
use crate::v2::sarzak::types::object::Object;

// Referent imports
use crate::v2::drawing::types::object_edge::ObjectEdge;

use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-struct-documentation"}}}
/// Render a rectangle
///
/// This represents additional information that is necessary to draw an Object in the user interface
///.
///
/// Note that although we are logically related to [Edge] via `R14` we actually render our own
/// edges. We use the svg rect primitive to do this.
///
/// I’m throwing this in for the fuck of it. I don’t know if it’ll be useful or not.
///
/// ```js
/// var rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
/// rect.setAttribute('class', 'objectRect');
/// rect.setAttribute('id', obj.id);
/// rect.setAttribute('x', obj.x);
/// rect.setAttribute('y', obj.y);
/// rect.setAttribute('width', obj.width);
///  rect.setAttribute('height', obj.height);
/// ```
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectUi {
    pub height: i64,
    pub id: Uuid,
    pub width: i64,
    /// R1: [`ObjectUi`] 'contains additional attributes to render' [`Object`]
    pub object_id: Uuid,
    /// R13: [`ObjectUi`] 'has an origin represented by a' [`Point`]
    pub origin: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-implementation"}}}
impl ObjectUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-struct-impl-new"}}}
    /// Inter a new ObjectUi in the store, and return it's `id`.
    pub fn new(
        height: i64,
        width: i64,
        object_id: &Object,
        origin: &Point,
        store: &mut DrawingStore,
    ) -> ObjectUi {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{}:{:?}:{:?}", height, width, object_id, origin).as_bytes(),
        );
        let new = ObjectUi {
            height: height,
            width: width,
            object_id: object_id.id,
            origin: origin.id,
            id,
        };
        store.inter_object_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-struct-impl-nav-forward-to-object_id"}}}
    /// Navigate to [`Object`] across R1(1-?)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-struct-impl-nav-forward-to-origin"}}}
    /// Navigate to [`Point`] across R13(1-?)
    pub fn r13_point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        vec![store.exhume_point(&self.origin).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_ui-struct-impl-nav-backward-1_M-to-object_edge"}}}
    /// Navigate to [`ObjectEdge`] across R18(1-M)
    pub fn r18_object_edge<'a>(&'a self, store: &'a DrawingStore) -> Vec<&ObjectEdge> {
        store
            .iter_object_edge()
            .filter_map(|object_edge| {
                if object_edge.oui_id == self.id {
                    Some(object_edge)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
