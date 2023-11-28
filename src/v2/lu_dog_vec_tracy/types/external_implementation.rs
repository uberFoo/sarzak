// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"external_implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::body::Body;
use crate::v2::lu_dog_vec_tracy::types::body::BodyEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-documentation"}}}
/// Some extern source of the function’s body.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalImplementation {
    pub function: String,
    pub id: usize,
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
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<ExternalImplementation>> {
        store.inter_external_implementation(|id| {
            Rc::new(RefCell::new(ExternalImplementation {
                function: function.to_owned(),
                id,
                x_model: x_model.to_owned(),
                object: object.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub fn r80_body<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Body>>> {
        span!("r80_body");
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external_implementation-implementation"}}}
impl PartialEq for ExternalImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.function == other.function
            && self.x_model == other.x_model
            && self.object == other.object
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
