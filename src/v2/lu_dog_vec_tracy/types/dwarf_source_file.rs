// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"dwarf_source_file-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::item::Item;
use crate::v2::lu_dog_vec_tracy::types::span::Span;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
    pub fn new(source: String, store: &mut LuDogVecTracyStore) -> Rc<RefCell<DwarfSourceFile>> {
        store.inter_dwarf_source_file(|id| {
            Rc::new(RefCell::new(DwarfSourceFile {
                id,
                source: source.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-nav-backward-1_M-to-item"}}}
    /// Navigate to [`Item`] across R25(1-M)
    pub fn r25_item<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Item>>> {
        span!("r25_item");
        store
            .iter_item()
            .filter(|item| item.borrow().source == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"dwarf_source_file-struct-impl-nav-backward-1_M-to-span"}}}
    /// Navigate to [`Span`] across R64(1-M)
    pub fn r64_span<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Span>>> {
        span!("r64_span");
        store
            .iter_span()
            .filter(|span| span.borrow().source == self.id)
            .collect()
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
