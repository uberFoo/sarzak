// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_wrapper-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-documentation"}}}
/// The purpose of this object is to wrap `Object`. We need to be able to store a referential
///  attribute to the `ObjectStore`, and we can’t/don’t want to add that to `Object`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectWrapper {
    pub id: Uuid,
    /// R78: [`Object`] '🚧 Comments are out of order — see sarzak#14.' [`Object`]
    pub object: Uuid,
    /// R78: [`ZObjectStore`] '🚧 Comments are out of order — see sarzak#14.' [`ZObjectStore`]
    pub z_store: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-implementation"}}}
impl ObjectWrapper {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-impl-new"}}}
    /// Inter a new 'Object Wrapper' in the store, and return it's `id`.
    pub fn new(
        object: &Object,
        z_store: &Rc<RefCell<ZObjectStore>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ObjectWrapper>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ObjectWrapper {
            id,
            object: object.id,
            z_store: z_store.borrow().id,
        }));
        store.inter_object_wrapper(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-impl-nav-forward-assoc-to-object"}}}
    /// Navigate to [`Object`] across R78(1-*)
    pub fn r78_object<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<Object>>> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_wrapper-struct-impl-nav-forward-assoc-to-z_store"}}}
    /// Navigate to [`ZObjectStore`] across R78(1-*)
    pub fn r78_z_object_store<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<ZObjectStore>>> {
        vec![store.exhume_z_object_store(&self.z_store).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
