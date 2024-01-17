// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::enumeration::Enumeration;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumGeneric {
    pub id: Uuid,
    pub name: String,
    /// R104: [`EnumGeneric`] 'parameterizes' [`Enumeration`]
    pub woog_enum: Uuid,
    /// R106: [`EnumGeneric`] 'next' [`EnumGeneric`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl EnumGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-new"}}}
    /// Inter a new 'Enum Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        woog_enum: &Rc<RefCell<Enumeration>>,
        next: Option<&Rc<RefCell<EnumGeneric>>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<EnumGeneric>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(EnumGeneric {
            id,
            name,
            woog_enum: woog_enum.borrow().id,
            next: next.map(|enum_generic| enum_generic.borrow().id),
        }));
        store.inter_enum_generic(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R104(1-*)
    pub fn r104_enumeration<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Enumeration>>> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`EnumGeneric`] across R106(1-*c)
    pub fn r106_enum_generic<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<EnumGeneric>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_enum_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-bi-cond-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R106(1c-1c)
    pub fn r106c_enum_generic<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<EnumGeneric>>> {
        let enum_generic = store
            .iter_enum_generic()
            .find(|enum_generic| enum_generic.borrow().next == Some(self.id));
        match enum_generic {
            Some(ref enum_generic) => vec![enum_generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R105(1-1)
    pub fn r105_enumeration<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Enumeration>>> {
        vec![store
            .iter_enumeration()
            .find(|enumeration| enumeration.borrow().first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}