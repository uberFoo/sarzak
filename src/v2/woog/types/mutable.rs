// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"mutable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-const-documentation"}}}
/// Mutable
///
/// The type is declared as `mut`.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-const-definition"}}}
pub const MUTABLE: Uuid = uuid!["1a837379-51fe-51a5-baf1-e7bd8c22ef7a"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Mutable;

impl Mutable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        MUTABLE
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
