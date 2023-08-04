// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"visibility-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::access::Access;
use crate::v2::woog::types::krate::KRATE;
use crate::v2::woog::types::private::PRIVATE;
use crate::v2::woog::types::public::PUBLIC;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-enum-documentation"}}}
/// Item Visibility
///
/// This is a _very_ Rust-centric type. It represents the visibility levels that Rust surfaces
/// .
///
/// Private is the default, and requires no modifiers. Public is the most open, and indicated
///  by prefixing the item with "pub". In the middle is "pub(crate)", which makes the item public
///  within the crate.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Visibility {
    Krate(Uuid),
    Private(Uuid),
    Public(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-implementation"}}}
impl Visibility {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-new-impl"}}}
    /// Create a new instance of Visibility::Krate
    pub fn new_krate(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_visibility(&KRATE).unwrap()
    }

    /// Create a new instance of Visibility::Private
    pub fn new_private(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_visibility(&PRIVATE).unwrap()
    }

    /// Create a new instance of Visibility::Public
    pub fn new_public(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_visibility(&PUBLIC).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Krate(id) => *id,
            Self::Private(id) => *id,
            Self::Public(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-struct-impl-nav-backward-1_M-to-access"}}}
    /// Navigate to [`Access`] across R14(1-M)
    pub fn r14_access<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Access>>> {
        span!("r14_access");
        store
            .iter_access()
            .filter(|access| access.borrow().visibility == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
