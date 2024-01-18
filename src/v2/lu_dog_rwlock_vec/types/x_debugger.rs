// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_debugger-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_debugger-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_debugger-const-documentation"}}}
/// An expresision to invoke the debugger;
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_debugger-const-definition"}}}
pub const X_DEBUGGER: Uuid = uuid!["0a02fb20-b343-530c-9467-1cb2d6957339"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XDebugger;

impl XDebugger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        X_DEBUGGER
    }
}

impl Default for XDebugger {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
