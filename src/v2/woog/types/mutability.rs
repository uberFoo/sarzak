// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"mutability-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog::types::borrowed::BORROWED;
use crate::v2::woog::types::mutable::MUTABLE;
use crate::v2::woog::types::owned::OWNED;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-enum-documentation"}}}
/// Type Mutability
///
/// This is tied closely with Rust. There are two possible options: mutable and borrowed.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Mutability {
    Borrowed(Uuid),
    Mutable(Uuid),
    Owned(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-implementation"}}}
impl Mutability {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-new-impl"}}}
    /// Create a new instance of Mutability::Borrowed
    pub fn new_borrowed() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Borrowed(BORROWED)
    }

    /// Create a new instance of Mutability::Mutable
    pub fn new_mutable() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Mutable(MUTABLE)
    }

    /// Create a new instance of Mutability::Owned
    pub fn new_owned() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Owned(OWNED)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Mutability::Borrowed(id) => *id,
            Mutability::Mutable(id) => *id,
            Mutability::Owned(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
