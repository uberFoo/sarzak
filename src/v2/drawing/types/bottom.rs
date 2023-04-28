// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"bottom-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bottom-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bottom-const-documentation"}}}
/// The bottom of a rendered box
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bottom-const-definition"}}}
pub const BOTTOM: Uuid = uuid!["2d05ae4a-b681-59d9-8d79-4ea372cc11f1"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Bottom;

impl Bottom {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        BOTTOM
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
