// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"visibility-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-enum-documentation"}}}
/// Item Visibility
///
/// This is a _very_ Rust-centric type. It represents the visibility levels that Rust surfaces
///.
///
/// Private is the default, and requires no modifiers. Public is the most open, and indicated
/// by prefixing the item with "pub". In the middle is "pub(crate)", which makes the item public
/// within the crate.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Visibility {
    Krate(Uuid),
    Private(Uuid),
    Public(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-implementation"}}}
impl Visibility {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"visibility-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Visibility::Krate(id) => *id,
            Visibility::Private(id) => *id,
            Visibility::Public(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
