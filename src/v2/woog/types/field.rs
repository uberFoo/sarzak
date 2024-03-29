// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::enumeration_field::EnumerationField;
use crate::v2::woog::types::grace_type::GraceType;
use crate::v2::woog::types::structure_field::StructureField;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
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
    pub fn new(
        name: String,
        ty: &Arc<RwLock<GraceType>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Field>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Field {
            id,
            name,
            ty: ty.read().unwrap().id(),
        }));
        store.inter_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`GraceType`] across R29(1-*)
    pub fn r29_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<GraceType>>> {
        vec![store.exhume_grace_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field-struct-impl-nav-backward-assoc-one-cond-to-structure_field"}}}
    /// Navigate to [`StructureField`] across R27(1-1c)
    pub fn r27_structure_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<StructureField>>> {
        let structure_field = store
            .iter_structure_field()
            .find(|structure_field| structure_field.read().unwrap().woog_struct == self.id);
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
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<EnumerationField>>> {
        let enumeration_field = store
            .iter_enumeration_field()
            .find(|enumeration_field| enumeration_field.read().unwrap().woog_enum == self.id);
        match enumeration_field {
            Some(enumeration_field) => vec![enumeration_field],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
