// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ownership-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-use-statements"}}}
use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
use crate::v2::woog_single::types::access::Access;
use crate::v2::woog_single::types::borrowed::BORROWED;
use crate::v2::woog_single::types::mutable::MUTABLE;
use crate::v2::woog_single::types::owned::OWNED;
use serde::{Deserialize, Serialize};
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
    pub fn new_borrowed() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Borrowed(BORROWED)
    }

    /// Create a new instance of Ownership::Mutable
    pub fn new_mutable() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Mutable(MUTABLE)
    }

    /// Create a new instance of Ownership::Owned
    pub fn new_owned() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Owned(OWNED)
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
    pub fn r15_access<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Access> {
        store
            .iter_access()
            .filter(|access| access.ownership == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
