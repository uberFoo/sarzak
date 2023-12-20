// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::call::Call;
use crate::v2::lu_dog_vec::types::call::CallEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FunctionCall {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-implementation"}}}
impl FunctionCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-new"}}}
    /// Inter a new 'Function Call' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogVecStore) -> Rc<RefCell<FunctionCall>> {
        store.inter_function_call(|id| {
            Rc::new(RefCell::new(FunctionCall {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-nav-forward-to-func"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub fn r30_call<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Call>>> {
        vec![store
            .iter_call()
            .find(|call| {
                if let CallEnum::FunctionCall(id) = call.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-implementation"}}}
impl PartialEq for FunctionCall {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
