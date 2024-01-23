// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"func_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::function::Function;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-documentation"}}}
/// These are generics associated with a function.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FuncGeneric {
    pub id: usize,
    pub name: String,
    /// R107: [`FuncGeneric`] '' [`Function`]
    pub func: Option<usize>,
    /// R3: [`FuncGeneric`] '' [`FuncGeneric`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-implementation"}}}
impl FuncGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-new"}}}
    /// Inter a new 'Func Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        func: Option<&Arc<RwLock<Function>>>,
        next: Option<&Arc<RwLock<FuncGeneric>>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<FuncGeneric>> {
        store.inter_func_generic(|id| {
            Arc::new(RwLock::new(FuncGeneric {
                id,
                name: name.to_owned(),
                func: func.map(|function| function.read().unwrap().id),
                next: next.map(|func_generic| func_generic.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-to-func"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-func"}}}
    /// Navigate to [`Function`] across R107(1-*c)
    pub fn r107_function<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        match self.func {
            Some(ref func) => vec![store.exhume_function(&func).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FuncGeneric`] across R3(1-*c)
    pub fn r3_func_generic<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<FuncGeneric>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_func_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-bi-cond-to-func_generic"}}}
    /// Navigate to [`FuncGeneric`] across R3(1c-1c)
    pub fn r3c_func_generic<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<FuncGeneric>>> {
        let func_generic = store
            .iter_func_generic()
            .find(|func_generic| func_generic.read().unwrap().next == Some(self.id));
        match func_generic {
            Some(ref func_generic) => vec![func_generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-to-function"}}}
    /// Navigate to [`Function`] across R99(1-1)
    pub fn r99_function<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        vec![store
            .iter_function()
            .find(|function| function.read().unwrap().first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::FuncGeneric(id) = value_type.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-implementation"}}}
impl PartialEq for FuncGeneric {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.func == other.func && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
