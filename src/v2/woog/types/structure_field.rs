// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"structure_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::field::Field;
use crate::v2::woog::types::structure::Structure;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-documentation"}}}
/// A Field in a Structure
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructureField {
    pub id: Uuid,
    /// R30: [`StructureField`] 'came before' [`StructureField`]
    pub next: Option<Uuid>,
    /// R27: [`Field`] '🚧 Comments are out of order — see sarzak#14.' [`Field`]
    pub woog_struct: Uuid,
    /// R27: [`Structure`] '🚧 Comments are out of order — see sarzak#14.' [`Structure`]
    pub field: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-implementation"}}}
impl StructureField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-new"}}}
    /// Inter a new 'Structure Field' in the store, and return it's `id`.
    pub fn new(
        next: Option<&Arc<RwLock<StructureField>>>,
        woog_struct: &Arc<RwLock<Field>>,
        field: &Arc<RwLock<Structure>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<StructureField>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(StructureField {
            id,
            next: next.map(|structure_field| structure_field.read().unwrap().id),
            woog_struct: woog_struct.read().unwrap().id,
            field: field.read().unwrap().id,
        }));
        store.inter_structure_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`StructureField`] across R30(1-*c)
    pub fn r30_structure_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<StructureField>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_structure_field(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-backward-one-bi-cond-to-structure_field"}}}
    /// Navigate to [`StructureField`] across R30(1c-1c)
    pub fn r30c_structure_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<Arc<RwLock<StructureField>>> {
        let structure_field = store
            .iter_structure_field()
            .find(|structure_field| structure_field.read().unwrap().next == Some(self.id));
        match structure_field {
            Some(ref structure_field) => vec![structure_field.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-woog_struct"}}}
    /// Navigate to [`Field`] across R27(1-*)
    pub fn r27_field<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Field>>> {
        vec![store.exhume_field(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-field"}}}
    /// Navigate to [`Structure`] across R27(1-*)
    pub fn r27_structure<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Structure>>> {
        vec![store.exhume_structure(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
