// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"func_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
        func: Option<&Rc<RefCell<Function>>>,
        next: Option<&Rc<RefCell<FuncGeneric>>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<FuncGeneric>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(FuncGeneric {
            id,
            name,
            func: func.map(|function| function.borrow().id),
            next: next.map(|func_generic| func_generic.borrow().id),
        }));
        store.inter_func_generic(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-to-func"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-func"}}}
    /// Navigate to [`Function`] across R107(1-*c)
    pub fn r107_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        match self.func {
            Some(ref func) => vec![store.exhume_function(&func).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FuncGeneric`] across R3(1-*c)
    pub fn r3_func_generic<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<FuncGeneric>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_func_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-bi-cond-to-func_generic"}}}
    /// Navigate to [`FuncGeneric`] across R3(1c-1c)
    pub fn r3c_func_generic<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<FuncGeneric>>> {
        let func_generic = store
            .iter_func_generic()
            .find(|func_generic| func_generic.borrow().next == Some(self.id));
        match func_generic {
            Some(ref func_generic) => vec![func_generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-to-function"}}}
    /// Navigate to [`Function`] across R99(1-1)
    pub fn r99_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        vec![store
            .iter_function()
            .find(|function| function.borrow().first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
