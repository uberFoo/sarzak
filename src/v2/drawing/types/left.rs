// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"left-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"left-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"left-const-documentation"}}}
/// The left side of a rendered box
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"left-const-definition"}}}
pub const LEFT: Uuid = uuid!["52636bac-3f47-5792-8a32-166dbe8af74f"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Left;

impl Left {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        LEFT
    }
}

impl Default for Left {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
