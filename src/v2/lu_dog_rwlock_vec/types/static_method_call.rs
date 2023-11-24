// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"static_method_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::call::Call;
use crate::v2::lu_dog_rwlock_vec::types::call::CallEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-documentation"}}}
/// A Static Method Call
///
/// This is when you call a function on the type (struct) itself. There is no instance involved
///  in this, although it may return an instance.
///
/// The name attribute is the name of the static method.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StaticMethodCall {
    pub func: String,
    pub id: usize,
    pub ty: String,
    pub unique: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-implementation"}}}
impl StaticMethodCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-impl-new"}}}
    /// Inter a new 'Static Method Call' in the store, and return it's `id`.
    pub fn new(
        func: String,
        ty: String,
        unique: Uuid,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<StaticMethodCall>> {
        store.inter_static_method_call(|id| {
            Arc::new(RwLock::new(StaticMethodCall {
                func: func.to_owned(),
                id,
                ty: ty.to_owned(),
                unique,
            }))
        })
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-impl-nav-backward-1_Mc-to-tuple_field"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub fn r30_call<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Call>>> {
        vec![store
            .iter_call()
            .find(|call| {
                if let CallEnum::StaticMethodCall(id) = call.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-implementation"}}}
impl PartialEq for StaticMethodCall {
    fn eq(&self, other: &Self) -> bool {
        self.func == other.func && self.ty == other.ty && self.unique == other.unique
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
