// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"dwarf_source_file-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::item::Item;
use crate::v2::lu_dog_rwlock::types::span::Span;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DwarfSourceFile {
    pub id: Uuid,
    pub source: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-implementation"}}}
impl DwarfSourceFile {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-new"}}}
    /// Inter a new 'Dwarf Source File' in the store, and return it's `id`.
    pub fn new(source: String, store: &mut LuDogRwlockStore) -> Arc<RwLock<DwarfSourceFile>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(DwarfSourceFile { id, source }));
        store.inter_dwarf_source_file(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-nav-backward-1_M-to-item"}}}
    /// Navigate to [`Item`] across R25(1-M)
    pub fn r25_item<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Item>>> {
        store
            .iter_item()
            .filter(|item| item.read().unwrap().source == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-nav-backward-1_M-to-span"}}}
    /// Navigate to [`Span`] across R64(1-M)
    pub fn r64_span<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Span>>> {
        store
            .iter_span()
            .filter(|span| span.read().unwrap().source == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
