// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::enumeration::Enumeration;
use crate::v2::woog::types::field::Field;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-documentation"}}}
/// A Field in an Enumeration
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumerationField {
    pub id: Uuid,
    /// R28: [`Enumeration`] '🚧 Comments are out of order — see sarzak#14.' [`Enumeration`]
    pub field: Uuid,
    /// R28: [`Field`] '🚧 Comments are out of order — see sarzak#14.' [`Field`]
    pub woog_enum: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-implementation"}}}
impl EnumerationField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-impl-new"}}}
    /// Inter a new 'Enumeration Field' in the store, and return it's `id`.
    pub fn new(
        field: &Arc<RwLock<Enumeration>>,
        woog_enum: &Arc<RwLock<Field>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<EnumerationField>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(EnumerationField {
            id,
            field: field.read().unwrap().id,
            woog_enum: woog_enum.read().unwrap().id,
        }));
        store.inter_enumeration_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-impl-nav-forward-assoc-to-field"}}}
    /// Navigate to [`Enumeration`] across R28(1-*)
    pub fn r28_enumeration<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Enumeration>>> {
        vec![store.exhume_enumeration(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-impl-nav-forward-assoc-to-woog_enum"}}}
    /// Navigate to [`Field`] across R28(1-*)
    pub fn r28_field<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Field>>> {
        vec![store.exhume_field(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
