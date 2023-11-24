// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_rwlock_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_rwlock_vec::types::function::Function;
use crate::v2::lu_dog_rwlock_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_rwlock_vec::types::import::Import;
use crate::v2::lu_dog_rwlock_vec::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_rwlock_vec::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
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
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Enumeration>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Item>> {
        store.inter_item(|id| {
            Arc::new(RwLock::new(Item {
                source: source.read().unwrap().id,
                subtype: ItemEnum::Enumeration(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_function(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Item>> {
        store.inter_item(|id| {
            Arc::new(RwLock::new(Item {
                source: source.read().unwrap().id,
                subtype: ItemEnum::Function(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation_block"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation_block(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<ImplementationBlock>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Item>> {
        store.inter_item(|id| {
            Arc::new(RwLock::new(Item {
                source: source.read().unwrap().id,
                subtype: ItemEnum::ImplementationBlock(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_import(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Item>> {
        store.inter_item(|id| {
            Arc::new(RwLock::new(Item {
                source: source.read().unwrap().id,
                subtype: ItemEnum::Import(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_x_macro(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<XMacro>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Item>> {
        store.inter_item(|id| {
            Arc::new(RwLock::new(Item {
                source: source.read().unwrap().id,
                subtype: ItemEnum::XMacro(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_woog_struct(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Item>> {
        store.inter_item(|id| {
            Arc::new(RwLock::new(Item {
                source: source.read().unwrap().id,
                subtype: ItemEnum::WoogStruct(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<DwarfSourceFile>>> {
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
