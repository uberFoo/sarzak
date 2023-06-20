// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::implementation::Implementation;
use crate::v2::lu_dog_async::types::import::Import;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    WoogStruct(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_function"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_function(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().await.id,
            subtype: ItemEnum::Function(subtype.read().await.id),
            id,
        }));
        store.inter_item(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_implementation"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_implementation(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Implementation>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().await.id,
            subtype: ItemEnum::Implementation(subtype.read().await.id),
            id,
        }));
        store.inter_item(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_import"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_import(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().await.id,
            subtype: ItemEnum::Import(subtype.read().await.id),
            id,
        }));
        store.inter_item(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-new_woog_struct"}}}
    /// Inter a new Item in the store, and return it's `id`.
    pub async fn new_woog_struct(
        source: &Arc<RwLock<DwarfSourceFile>>,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Item>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Item {
            source: source.read().await.id,
            subtype: ItemEnum::WoogStruct(subtype.read().await.id),
            id,
        }));
        store.inter_item(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R25(1-*)
    pub async fn r25_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<DwarfSourceFile>>> {
        span!("r25_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
