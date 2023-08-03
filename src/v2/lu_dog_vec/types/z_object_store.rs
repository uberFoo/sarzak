// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_object_store-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vec::types::object_wrapper::ObjectWrapper;
use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-documentation"}}}
/// A generated ObjectStore
///
/// This is the backing store for the structs.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZObjectStore {
    pub domain: String,
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-implementation"}}}
impl ZObjectStore {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-impl-new"}}}
    /// Inter a new 'Object Store' in the store, and return it's `id`.
    pub fn new(
        domain: String,
        name: String,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ZObjectStore>> {
        store.inter_z_object_store(|id| {
            Rc::new(RefCell::new(ZObjectStore {
                domain: domain.to_owned(),
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-impl-nav-forward-cond-to-implementation"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-impl-nav-backward-one-bi-cond-to-implementation_block"}}}
    /// Navigate to [`ImplementationBlock`] across R83(1c-1c)
    pub fn r83c_implementation_block<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ImplementationBlock>>> {
        span!("r83_implementation_block");
        let implementation_block = store
            .iter_implementation_block()
            .find(|implementation_block| {
                implementation_block.borrow().object_store == Some(self.id)
            });
        match implementation_block {
            Some(ref implementation_block) => vec![implementation_block.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-struct-impl-nav-backward-assoc-many-to-object_wrapper"}}}
    /// Navigate to [`ObjectWrapper`] across R78(1-M)
    pub fn r78_object_wrapper<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ObjectWrapper>>> {
        span!("r78_object_wrapper");
        store
            .iter_object_wrapper()
            .filter(|object_wrapper| object_wrapper.borrow().z_store == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::ZObjectStore(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_object_store-implementation"}}}
impl PartialEq for ZObjectStore {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
