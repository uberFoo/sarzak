// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"private-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"private-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"private-const-documentation"}}}
/// Private Visibility
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"private-const-definition"}}}
pub const PRIVATE: Uuid = uuid!["bb92c265-039b-5f30-94d8-a1367ac0f895"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Private;

impl Private {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        PRIVATE
    }
}

impl Default for Private {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
