// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_vanilla::types::enumeration::Enumeration;
use crate::v2::lu_dog_vanilla::types::function::Function;
use crate::v2::lu_dog_vanilla::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vanilla::types::import::Import;
use crate::v2::lu_dog_vanilla::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vanilla::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
    Enumeration(Uuid),
    Function(Uuid),
    ImplementationBlock(Uuid),
    Import(Uuid),
    XMacro(Uuid),
    WoogStruct(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_enumeration"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_enumeration(
        source: &DwarfSourceFile,
        subtype: &Enumeration,
        store: &mut LuDogVanillaStore,
    ) -> Item {
        let id = Uuid::new_v4();
        let new = Item {
            source: source.id,
            subtype: ItemEnum::Enumeration(subtype.id),
            id,
        };
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_function(
        source: &DwarfSourceFile,
        subtype: &Function,
        store: &mut LuDogVanillaStore,
    ) -> Item {
        let id = Uuid::new_v4();
        let new = Item {
            source: source.id,
            subtype: ItemEnum::Function(subtype.id),
            id,
        };
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation_block"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_implementation_block(
        source: &DwarfSourceFile,
        subtype: &ImplementationBlock,
        store: &mut LuDogVanillaStore,
    ) -> Item {
        let id = Uuid::new_v4();
        let new = Item {
            source: source.id,
            subtype: ItemEnum::ImplementationBlock(subtype.id),
            id,
        };
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_import(
        source: &DwarfSourceFile,
        subtype: &Import,
        store: &mut LuDogVanillaStore,
    ) -> Item {
        let id = Uuid::new_v4();
        let new = Item {
            source: source.id,
            subtype: ItemEnum::Import(subtype.id),
            id,
        };
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_x_macro(
        source: &DwarfSourceFile,
        subtype: &XMacro,
        store: &mut LuDogVanillaStore,
    ) -> Item {
        let id = Uuid::new_v4();
        let new = Item {
            source: source.id,
            subtype: ItemEnum::XMacro(subtype.id),
            id,
        };
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub fn new_woog_struct(
        source: &DwarfSourceFile,
        subtype: &WoogStruct,
        store: &mut LuDogVanillaStore,
    ) -> Item {
        let id = Uuid::new_v4();
        let new = Item {
            source: source.id,
            subtype: ItemEnum::WoogStruct(subtype.id),
            id,
        };
        store.inter_item(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&DwarfSourceFile> {
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
