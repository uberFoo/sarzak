// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"supertype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::sarzak::types::isa::Isa;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-struct-documentation"}}}
/// This object represents the *supertype* in a *supertype-subtype*
/// relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Supertype {
    pub id: Uuid,
    /// R14: [`Supertype`] 'is an instance of an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-implementation"}}}
impl Supertype {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-struct-impl-new"}}}
    /// Inter a new 'Supertype' in the store, and return it's `id`.
    pub fn new(obj_id: &Arc<RwLock<Object>>, store: &mut SarzakStore) -> Arc<RwLock<Supertype>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Supertype {
            id,
            obj_id: obj_id.read().unwrap().id,
        }));
        store.inter_supertype(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R14(1-*)
    pub fn r14_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<Arc<RwLock<Object>>> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"supertype-struct-impl-nav-backward-one-to-isa"}}}
    /// Navigate to [`Isa`] across R13(1-1)
    pub fn r13_isa<'a>(&'a self, store: &'a SarzakStore) -> Vec<Arc<RwLock<Isa>>> {
        vec![store
            .iter_isa()
            .find(|isa| isa.read().unwrap().supertype == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
