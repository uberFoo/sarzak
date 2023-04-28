// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"krate-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"krate-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"krate-const-documentation"}}}
/// Crate Visibility
///
/// The item is visibile within the crate.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"krate-const-definition"}}}
pub const KRATE: Uuid = uuid!["7de289c9-3492-507f-b3ab-9cdbecb7e4a3"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Krate;

impl Krate {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        KRATE
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
