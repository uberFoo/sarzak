// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation_block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::enumeration::Enumeration;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::item::Item;
use crate::v2::lu_dog::types::item::ItemEnum;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use crate::v2::lu_dog::types::z_object_store::ZObjectStore;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ImplementationBlock {
    pub id: Uuid,
    /// R84: [`ImplementationBlock`] 'may exist for an' [`Enumeration`]
    pub enumeration: Option<Uuid>,
    /// R8: [`ImplementationBlock`] 'adds functions to a' [`WoogStruct`]
    pub model_type: Option<Uuid>,
    /// R83: [`ImplementationBlock`] 'may refer to an' [`ZObjectStore`]
    pub object_store: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-implementation"}}}
impl ImplementationBlock {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-new"}}}
    /// Inter a new 'Implementation Block' in the store, and return it's `id`.
    pub fn new(
        enumeration: Option<&Rc<RefCell<Enumeration>>>,
        model_type: Option<&Rc<RefCell<WoogStruct>>>,
        object_store: Option<&Rc<RefCell<ZObjectStore>>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ImplementationBlock>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ImplementationBlock {
            id,
            enumeration: enumeration.map(|enumeration| enumeration.borrow().id),
            model_type: model_type.map(|woog_struct| woog_struct.borrow().id),
            object_store: object_store.map(|z_object_store| z_object_store.borrow().id),
        }));
        store.inter_implementation_block(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-to-model_type"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-cond-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R84(1-*c)
    pub fn r84_enumeration<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Enumeration>>> {
        match self.enumeration {
            Some(ref enumeration) => vec![store.exhume_enumeration(&enumeration).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-cond-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*c)
    pub fn r8_woog_struct<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<WoogStruct>>> {
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
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<ZObjectStore>>> {
        match self.object_store {
            Some(ref object_store) => vec![store.exhume_z_object_store(&object_store).unwrap()],
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-one-bi-cond-to-enumeration"}}}
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub fn r9_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        store
            .iter_function()
            .filter(|function| function.borrow().impl_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Item>>> {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
