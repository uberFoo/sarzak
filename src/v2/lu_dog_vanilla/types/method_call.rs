// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"method_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::call::Call;
use crate::v2::lu_dog_vanilla::types::call::CallEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-documentation"}}}
/// A Method Call
///
/// This is when you call a function on an instance of a struct. The name attribute is the name
///  of the method.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MethodCall {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-implementation"}}}
impl MethodCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-struct-impl-new"}}}
    /// Inter a new 'Method Call' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogVanillaStore) -> MethodCall {
        let id = Uuid::new_v4();
        let new = MethodCall { id, name };
        store.inter_method_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"method_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub fn r30_call<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Call> {
        vec![store
            .iter_call()
            .find(|call| {
                if let CallEnum::MethodCall(id) = call.subtype {
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
