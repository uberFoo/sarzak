// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"dwarf_source_file-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::span::Span;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-documentation"}}}
/// The Source Code
///
/// The main purpose of this object is to capture the source code that contains the definitions
///  that will be parsed. This allows us to do better error reporting in the interpreter. We
///  may also be able to do something about displaying compiled functions, maybe.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DwarfSourceFile {
    pub id: usize,
    pub source: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-implementation"}}}
impl DwarfSourceFile {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-new"}}}
    /// Inter a new 'Dwarf Source File' in the store, and return it's `id`.
    pub async fn new(source: String, store: &mut LuDogAsyncStore) -> Arc<RwLock<DwarfSourceFile>> {
        store
            .inter_dwarf_source_file(|id| {
                Arc::new(RwLock::new(DwarfSourceFile {
                    id,
                    source: source.to_owned(),
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-nav-backward-1_M-to-item"}}}
    /// Navigate to [`Item`] across R25(1-M)
    pub async fn r25_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r25_item");
        store
            .iter_item()
            .await
            .filter_map(|item| async {
                if item.read().await.source == self.id {
                    Some(item)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-nav-backward-1_M-to-span"}}}
    /// Navigate to [`Span`] across R64(1-M)
    pub async fn r64_span<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Span>>> {
        span!("r64_span");
        store
            .iter_span()
            .await
            .filter_map(|span| async {
                if span.read().await.source == self.id {
                    Some(span)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-implementation"}}}
impl PartialEq for DwarfSourceFile {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
