// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_async::types::enumeration::Enumeration;
use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_async::types::import::Import;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_async::types::x_macro::XMacro;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    pub async fn new_enumeration(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Enumeration>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let s_id = subtype.read().await.id;
        let source = source.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_item(|id| {
                Arc::new(RwLock::new(Item {
                    source, // (b)
                    subtype: ItemEnum::Enumeration(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_function(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let s_id = subtype.read().await.id;
        let source = source.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_item(|id| {
                Arc::new(RwLock::new(Item {
                    source, // (b)
                    subtype: ItemEnum::Function(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation_block"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_implementation_block(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<ImplementationBlock>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let s_id = subtype.read().await.id;
        let source = source.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_item(|id| {
                Arc::new(RwLock::new(Item {
                    source, // (b)
                    subtype: ItemEnum::ImplementationBlock(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_import(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let s_id = subtype.read().await.id;
        let source = source.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_item(|id| {
                Arc::new(RwLock::new(Item {
                    source, // (b)
                    subtype: ItemEnum::Import(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_x_macro"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_x_macro(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<XMacro>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let s_id = subtype.read().await.id;
        let source = source.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_item(|id| {
                Arc::new(RwLock::new(Item {
                    source, // (b)
                    subtype: ItemEnum::XMacro(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_woog_struct(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let s_id = subtype.read().await.id;
        let source = source.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_item(|id| {
                Arc::new(RwLock::new(Item {
                    source, // (b)
                    subtype: ItemEnum::WoogStruct(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub async fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        stream::iter(vec![store.exhume_dwarf_source_file(&self.source).await.unwrap()].into_iter())
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
