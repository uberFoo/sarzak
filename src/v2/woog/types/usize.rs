// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"usize-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"usize-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"usize-const-documentation"}}}
/// `usize` the size of a pointer, index, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"usize-const-definition"}}}
pub const USIZE: Uuid = uuid!["618c6689-4b5e-52a0-88f4-294b825793e6"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Usize;

impl Usize {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        USIZE
    }
}

impl Default for Usize {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
