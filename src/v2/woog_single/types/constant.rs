// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"constant-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"constant-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog_single::types::item::Item;
use serde::{Deserialize, Serialize};

use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"constant-struct-documentation"}}}
/// A Constant
///
/// This is an inviolable memory, of a certain type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"constant-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Constant {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"constant-implementation"}}}
impl Constant {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"constant-struct-impl-new"}}}
    /// Inter a new 'Constant' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut WoogSingleStore) -> Constant {
        let id = Uuid::new_v4();
        let new = Constant { id, name };
        store.inter_constant(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"constant-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R26(isa)
    pub fn r26_item<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Item> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
