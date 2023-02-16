// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"mutability-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

// Referent imports
use crate::woog::types::parameter::Parameter;
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
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-implementation"}}}
impl Mutability {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutability-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Mutability::Borrowed(id) => *id,
            Mutability::Mutable(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
