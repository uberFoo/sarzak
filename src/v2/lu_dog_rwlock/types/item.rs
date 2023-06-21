// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_rwlock::types::function::Function;
use crate::v2::lu_dog_rwlock::types::implementation::Implementation;
use crate::v2::lu_dog_rwlock::types::import::Import;
use crate::v2::lu_dog_rwlock::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_rwlock::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().unwrap().id,
            subtype: ItemEnum::Function(subtype.read().unwrap().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Implementation>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().unwrap().id,
            subtype: ItemEnum::Implementation(subtype.read().unwrap().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_import(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().unwrap().id,
            subtype: ItemEnum::Import(subtype.read().unwrap().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_x_macro(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<XMacro>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().unwrap().id,
            subtype: ItemEnum::XMacro(subtype.read().unwrap().id),
            id,
        }));
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_woog_struct(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().unwrap().id,
            subtype: ItemEnum::WoogStruct(subtype.read().unwrap().id),
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
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<DwarfSourceFile>>> {
        span!("r25_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
