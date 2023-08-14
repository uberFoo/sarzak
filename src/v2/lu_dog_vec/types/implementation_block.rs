// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation_block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_vec::types::function::Function;
use crate::v2::lu_dog_vec::types::item::Item;
use crate::v2::lu_dog_vec::types::item::ItemEnum;
use crate::v2::lu_dog_vec::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vec::types::z_object_store::ZObjectStore;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImplementationBlock {
    pub id: usize,
    /// R8: [`ImplementationBlock`] 'adds functions to a' [`WoogStruct`]
    pub model_type: Option<usize>,
    /// R83: [`ImplementationBlock`] 'may refer to an' [`ZObjectStore`]
    pub object_store: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-implementation"}}}
impl ImplementationBlock {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-new"}}}
    /// Inter a new 'Implementation Block' in the store, and return it's `id`.
    pub fn new(
        model_type: Option<&Rc<RefCell<WoogStruct>>>,
        object_store: Option<&Rc<RefCell<ZObjectStore>>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ImplementationBlock>> {
        store.inter_implementation_block(|id| {
            Rc::new(RefCell::new(ImplementationBlock {
                id,
                model_type: model_type.map(|woog_struct| woog_struct.borrow().id),
                object_store: object_store.map(|z_object_store| z_object_store.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-to-model_type"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-cond-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*c)
    pub fn r8_woog_struct<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<WoogStruct>>> {
        span!("r8_woog_struct");
        match self.model_type {
            Some(ref model_type) => vec![store.exhume_woog_struct(&model_type).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-cond-to-object_store"}}}
    /// Navigate to [`ZObjectStore`] across R83(1-*c)
    pub fn r83_z_object_store<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ZObjectStore>>> {
        span!("r83_z_object_store");
        match self.object_store {
            Some(ref object_store) => vec![store.exhume_z_object_store(&object_store).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-one-bi-cond-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R84(1c-1c)
    pub fn r84c_enumeration<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<Enumeration>>> {
        span!("r84_enumeration");
        let enumeration = store
            .iter_enumeration()
            .find(|enumeration| enumeration.borrow().implementation == Some(self.id));
        match enumeration {
            Some(ref enumeration) => vec![enumeration.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub fn r9_function<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r9_function");
        store
            .iter_function()
            .filter(|function| function.borrow().impl_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-one-to-z_object_store"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::ImplementationBlock(id) = item.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-implementation"}}}
impl PartialEq for ImplementationBlock {
    fn eq(&self, other: &Self) -> bool {
        self.model_type == other.model_type && self.object_store == other.object_store
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
