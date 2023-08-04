// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ownership-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::access::Access;
use crate::v2::woog::types::borrowed::BORROWED;
use crate::v2::woog::types::mutable::MUTABLE;
use crate::v2::woog::types::owned::OWNED;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-documentation"}}}
/// Type Ownership
///
/// This is tied closely with Rust. There are tthree possible options: owned, mutable and borrowed
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Ownership {
    Borrowed(Uuid),
    Mutable(Uuid),
    Owned(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-implementation"}}}
impl Ownership {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-new-impl"}}}
    /// Create a new instance of Ownership::Borrowed
    pub fn new_borrowed(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_ownership(&BORROWED).unwrap()
    }

    /// Create a new instance of Ownership::Mutable
    pub fn new_mutable(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_ownership(&MUTABLE).unwrap()
    }

    /// Create a new instance of Ownership::Owned
    pub fn new_owned(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_ownership(&OWNED).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Borrowed(id) => *id,
            Self::Mutable(id) => *id,
            Self::Owned(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-nav-backward-1_M-to-access"}}}
    /// Navigate to [`Access`] across R15(1-M)
    pub fn r15_access<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Access>>> {
        span!("r15_access");
        store
            .iter_access()
            .filter(|access| access.borrow().ownership == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
