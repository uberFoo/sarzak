// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"external_implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::body::Body;
use crate::v2::lu_dog::types::body::BodyEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-documentation"}}}
/// Some extern source of the function’s body.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExternalImplementation {
    pub function: String,
    pub id: Uuid,
    pub x_model: String,
    pub object: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-implementation"}}}
impl ExternalImplementation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-impl-new"}}}
    /// Inter a new 'External Implementation' in the store, and return it's `id`.
    pub fn new(
        function: String,
        x_model: String,
        object: String,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ExternalImplementation>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ExternalImplementation {
            function,
            id,
            x_model,
            object,
        }));
        store.inter_external_implementation(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub fn r80_body<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Body>>> {
        vec![store
            .iter_body()
            .find(|body| {
                if let BodyEnum::ExternalImplementation(id) = body.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
