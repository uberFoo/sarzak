// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_vec_tracy::types::enumeration::Enumeration;
use crate::v2::lu_dog_vec_tracy::types::function::Function;
use crate::v2::lu_dog_vec_tracy::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vec_tracy::types::import::Import;
use crate::v2::lu_dog_vec_tracy::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vec_tracy::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub subtype: ItemEnum,
    pub id: usize,
    /// R25: [`Item`] '' [`DwarfSourceFile`]
    pub source: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ItemEnum {
    Enumeration(usize),
    Function(usize),
    ImplementationBlock(usize),
    Import(usize),
    XMacro(usize),
    WoogStruct(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_enumeration"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_enumeration(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::Enumeration(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_function(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::Function(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation_block"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation_block(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<ImplementationBlock>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::ImplementationBlock(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_import(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Import>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::Import(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_x_macro(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<XMacro>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::XMacro(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_woog_struct(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::WoogStruct(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<DwarfSourceFile>>> {
        span!("r25_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.source == other.source
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
