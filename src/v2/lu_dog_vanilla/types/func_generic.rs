// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"func_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-documentation"}}}
/// These are generics associated with a function.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FuncGeneric {
    pub id: Uuid,
    pub name: String,
    /// R107: [`FuncGeneric`] '' [`Function`]
    pub func: Option<Uuid>,
    /// R3: [`FuncGeneric`] '' [`FuncGeneric`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-implementation"}}}
impl FuncGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-new"}}}
    /// Inter a new 'Func Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        func: Option<&Function>,
        next: Option<&FuncGeneric>,
        store: &mut LuDogVanillaStore,
    ) -> FuncGeneric {
        let id = Uuid::new_v4();
        let new = FuncGeneric {
            id,
            name,
            func: func.as_ref().map(|function| function.id),
            next: next.as_ref().map(|func_generic| func_generic.id),
        };
        store.inter_func_generic(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-to-func"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-func"}}}
    /// Navigate to [`Function`] across R107(1-*c)
    pub fn r107_function<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Function> {
        match self.func {
            Some(ref func) => vec![store.exhume_function(func).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FuncGeneric`] across R3(1-*c)
    pub fn r3_func_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FuncGeneric> {
        match self.next {
            Some(ref next) => vec![store.exhume_func_generic(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-bi-cond-to-func_generic"}}}
    /// Navigate to [`FuncGeneric`] across R3(1c-1c)
    pub fn r3c_func_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FuncGeneric> {
        let func_generic = store
            .iter_func_generic()
            .find(|func_generic| func_generic.next == Some(self.id));
        match func_generic {
            Some(ref func_generic) => vec![func_generic],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-to-function"}}}
    /// Navigate to [`Function`] across R99(1-1)
    pub fn r99_function<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Function> {
        vec![store
            .iter_function()
            .find(|function| function.first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
