// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_pl_mutex::types::function::Function;
use crate::v2::lu_dog_pl_mutex::types::implementation::Implementation;
use crate::v2::lu_dog_pl_mutex::types::import::Import;
use crate::v2::lu_dog_pl_mutex::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_pl_mutex::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
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
        source: &Arc<Mutex<DwarfSourceFile>>,
        subtype: &Arc<Mutex<Function>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Item {
            source: source.lock().id,
            subtype: ItemEnum::Function(subtype.lock().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation(
        source: &Arc<Mutex<DwarfSourceFile>>,
        subtype: &Arc<Mutex<Implementation>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Item {
            source: source.lock().id,
            subtype: ItemEnum::Implementation(subtype.lock().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_import(
        source: &Arc<Mutex<DwarfSourceFile>>,
        subtype: &Arc<Mutex<Import>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Item {
            source: source.lock().id,
            subtype: ItemEnum::Import(subtype.lock().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_x_macro(
        source: &Arc<Mutex<DwarfSourceFile>>,
        subtype: &Arc<Mutex<XMacro>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Item {
            source: source.lock().id,
            subtype: ItemEnum::XMacro(subtype.lock().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_woog_struct(
        source: &Arc<Mutex<DwarfSourceFile>>,
        subtype: &Arc<Mutex<WoogStruct>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Item {
            source: source.lock().id,
            subtype: ItemEnum::WoogStruct(subtype.lock().id),
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
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<DwarfSourceFile>>> {
        span!("r25_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}