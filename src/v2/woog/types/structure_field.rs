// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"structure_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
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
        next: Option<&Rc<RefCell<StructureField>>>,
        woog_struct: &Rc<RefCell<Field>>,
        field: &Rc<RefCell<Structure>>,
        store: &mut WoogStore,
    ) -> Rc<RefCell<StructureField>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(StructureField {
            id,
            next: next.map(|structure_field| structure_field.borrow().id),
            woog_struct: woog_struct.borrow().id,
            field: field.borrow().id,
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
    ) -> Vec<Rc<RefCell<StructureField>>> {
        span!("r30_structure_field");
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
    ) -> Vec<Rc<RefCell<StructureField>>> {
        span!("r30_structure_field");
        let structure_field = store
            .iter_structure_field()
            .find(|structure_field| structure_field.borrow().next == Some(self.id));
        match structure_field {
            Some(ref structure_field) => vec![structure_field.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-woog_struct"}}}
    /// Navigate to [`Field`] across R27(1-*)
    pub fn r27_field<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Field>>> {
        span!("r27_field");
        vec![store.exhume_field(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-field"}}}
    /// Navigate to [`Structure`] across R27(1-*)
    pub fn r27_structure<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Structure>>> {
        span!("r27_structure");
        vec![store.exhume_structure(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
