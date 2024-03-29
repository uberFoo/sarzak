// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::enumeration_field::EnumerationField;
use crate::v2::woog::types::item::Item;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-documentation"}}}
/// An Enumerated Data Type
///
/// This has fields like a structure, but it's only allowed to contain a
/// single value at a time. This is also refered to as an algebraic
/// data type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Enumeration {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-implementation"}}}
impl Enumeration {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-new"}}}
    /// Inter a new 'Enumeration' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut WoogStore) -> Arc<RwLock<Enumeration>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Enumeration { id, name }));
        store.inter_enumeration(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-assoc-many-to-enumeration_field"}}}
    /// Navigate to [`EnumerationField`] across R28(1-M)
    pub fn r28_enumeration_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<EnumerationField>>> {
        store
            .iter_enumeration_field()
            .filter(|enumeration_field| enumeration_field.read().unwrap().field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R26(isa)
    pub fn r26_item<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Item>>> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
