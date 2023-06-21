// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::implementation::Implementation;
use crate::v2::lu_dog::types::import::Import;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use crate::v2::lu_dog::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Item {
    pub subtype: ItemEnum,
    pub id: Uuid,
    /// R25: [`Item`] '' [`DwarfSourceFile`]
    pub source: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ItemEnum {
    Function(Uuid),
    Implementation(Uuid),
    Import(Uuid),
    XMacro(Uuid),
    WoogStruct(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_function(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Item>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Item {
            source: source.borrow().id,
            subtype: ItemEnum::Function(subtype.borrow().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Implementation>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Item>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Item {
            source: source.borrow().id,
            subtype: ItemEnum::Implementation(subtype.borrow().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_import(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<Import>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Item>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Item {
            source: source.borrow().id,
            subtype: ItemEnum::Import(subtype.borrow().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_x_macro(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<XMacro>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Item>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Item {
            source: source.borrow().id,
            subtype: ItemEnum::XMacro(subtype.borrow().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_woog_struct(
        source: &Rc<RefCell<DwarfSourceFile>>,
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Item>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Item {
            source: source.borrow().id,
            subtype: ItemEnum::WoogStruct(subtype.borrow().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<DwarfSourceFile>>> {
        span!("r25_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
