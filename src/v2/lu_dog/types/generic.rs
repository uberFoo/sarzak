// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-documentation"}}}
/// This is a generic “type”.
///
/// It’s really a placeholder in the extruder/compiler. We’ll use it as a type declaration
/// , and then define a new type for each use.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Generic {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-implementation"}}}
impl Generic {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-impl-new"}}}
    /// Inter a new 'Generic' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogStore) -> Rc<RefCell<Generic>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Generic { id, name }));
        store.inter_generic(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
