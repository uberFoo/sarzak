// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"plugin-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-const-documentation"}}}
/// An external compilation unit that may be loaded at run time.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-const-definition"}}}
pub const PLUGIN: Uuid = uuid!["9c054bdc-7358-5532-a7a3-40694478fab3"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Plugin;

impl Plugin {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        PLUGIN
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
