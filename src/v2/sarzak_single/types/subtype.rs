// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-use-statements"}}}
use uuid::Uuid;

use crate::v2::sarzak_single::types::isa::Isa;
use crate::v2::sarzak_single::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak_single::store::ObjectStore as SarzakSingleStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-documentation"}}}
/// The *subtype* in a *supertype-subtype* relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Subtype {
    pub id: Uuid,
    /// R27: [`Subtype`] 'formalize an' [`Isa`]
    pub isa: Uuid,
    /// R15: [`Subtype`] 'is an instance of an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-implementation"}}}
impl Subtype {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-impl-new"}}}
    /// Inter a new 'Subtype' in the store, and return it's `id`.
    pub fn new(isa: &Isa, obj_id: &Object, store: &mut SarzakSingleStore) -> Subtype {
        let id = Uuid::new_v4();
        let new = Subtype {
            id,
            isa: isa.id,
            obj_id: obj_id.id,
        };
        store.inter_subtype(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-impl-nav-forward-to-isa"}}}
    /// Navigate to [`Isa`] across R27(1-*)
    pub fn r27_isa<'a>(&'a self, store: &'a SarzakSingleStore) -> Vec<&Isa> {
        vec![store.exhume_isa(&self.isa).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R15(1-*)
    pub fn r15_object<'a>(&'a self, store: &'a SarzakSingleStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
