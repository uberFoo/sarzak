// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog_2::UUID_NS;

// Referrer imports
use crate::v2::sarzak::types::object::Object;

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::woog_2::store::ObjectStore as Woog2Store;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-documentation"}}}
/// A Reference
///
/// Specifically this is a reference to something that was translated from an [`Object`]. In
/// rust that's a `struct` or an `enum`.
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Reference {
    pub id: Uuid,
    /// R13: [`Reference`] 'points at an' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl Reference {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new"}}}
    /// Inter a new Reference in the store, and return it's `id`.
    pub fn new(object: &Object, store: &mut Woog2Store) -> Reference {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", object).as_bytes());
        let new = Reference {
            object: object.id,
            id,
        };
        store.inter_reference(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R13(1-*)
    pub fn r13_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
