// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog_single::types::enumeration_field::EnumerationField;
use crate::v2::woog_single::types::grace_type::GraceType;
use crate::v2::woog_single::types::structure_field::StructureField;
use serde::{Deserialize, Serialize};

use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-documentation"}}}
/// A Field
///
/// A field is a named part of a data structure (an [`Enumeration`] or a [`Structure`] of a
///  given [`GraceType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Field {
    pub id: Uuid,
    pub name: String,
    /// R29: [`Field`] 'is of a' [`GraceType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-implementation"}}}
impl Field {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-new"}}}
    /// Inter a new 'Field' in the store, and return it's `id`.
    pub fn new(name: String, ty: &GraceType, store: &mut WoogSingleStore) -> Field {
        let id = Uuid::new_v4();
        let new = Field {
            id,
            name,
            ty: ty.id(),
        };
        store.inter_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`GraceType`] across R29(1-*)
    pub fn r29_grace_type<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&GraceType> {
        vec![store.exhume_grace_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-backward-assoc-one-cond-to-structure_field"}}}
    /// Navigate to [`StructureField`] across R27(1-1c)
    pub fn r27_structure_field<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&StructureField> {
        let structure_field = store
            .iter_structure_field()
            .find(|structure_field| structure_field.woog_struct == self.id);
        match structure_field {
            Some(structure_field) => vec![structure_field],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-backward-assoc-one-cond-to-enumeration_field"}}}
    /// Navigate to [`EnumerationField`] across R28(1-1c)
    pub fn r28_enumeration_field<'a>(
        &'a self,
        store: &'a WoogSingleStore,
    ) -> Vec<&EnumerationField> {
        let enumeration_field = store
            .iter_enumeration_field()
            .find(|enumeration_field| enumeration_field.woog_enum == self.id);
        match enumeration_field {
            Some(enumeration_field) => vec![enumeration_field],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
