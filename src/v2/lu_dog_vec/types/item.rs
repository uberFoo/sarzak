// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_vec::types::function::Function;
use crate::v2::lu_dog_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vec::types::import::Import;
use crate::v2::lu_dog_vec::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vec::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Item {
    pub subtype: ItemEnum,
    pub id: usize,
    /// R25: [`Item`] '' [`DwarfSourceFile`]
    pub source: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ItemEnum {
    Function(usize),
    ImplementationBlock(usize),
    Import(usize),
    XMacro(usize),
    WoogStruct(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_function(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::Function(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation_block"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation_block(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<ImplementationBlock>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::ImplementationBlock(subtype.borrow().id),
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
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::Import(subtype.borrow().id),
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
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::XMacro(subtype.borrow().id),
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
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Item>> {
        store.inter_item(|id| {
            Rc::new(RefCell::new(Item {
                source: source.borrow().id,
                subtype: ItemEnum::WoogStruct(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<DwarfSourceFile>>> {
        span!("r25_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
