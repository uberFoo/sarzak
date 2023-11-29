// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog_single::types::enumeration::Enumeration;
use crate::v2::woog_single::types::field::Field;
use serde::{Deserialize, Serialize};

use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
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
        field: &Enumeration,
        woog_enum: &Field,
        store: &mut WoogSingleStore,
    ) -> EnumerationField {
        let id = Uuid::new_v4();
        let new = EnumerationField {
            id,
            field: field.id,
            woog_enum: woog_enum.id,
        };
        store.inter_enumeration_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-impl-nav-forward-assoc-to-field"}}}
    /// Navigate to [`Enumeration`] across R28(1-*)
    pub fn r28_enumeration<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Enumeration> {
        vec![store.exhume_enumeration(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration_field-struct-impl-nav-forward-assoc-to-woog_enum"}}}
    /// Navigate to [`Field`] across R28(1-*)
    pub fn r28_field<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Field> {
        vec![store.exhume_field(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
