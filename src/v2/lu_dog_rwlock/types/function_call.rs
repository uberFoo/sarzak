// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::call::Call;
use crate::v2::lu_dog_rwlock::types::call::CallEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionCall {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-implementation"}}}
impl FunctionCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-new"}}}
    /// Inter a new 'Function Call' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogRwlockStore) -> Arc<RwLock<FunctionCall>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FunctionCall { id, name }));
        store.inter_function_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub fn r30_call<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Call>>> {
        vec![store
            .iter_call()
            .find(|call| {
                if let CallEnum::FunctionCall(id) = call.read().unwrap().subtype {
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
