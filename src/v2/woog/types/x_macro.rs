// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_macro-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-const-documentation"}}}
/// A Macro Invocation
///
/// This is here for completeness, and I doubt that I'll ever actually use it. See [the documentation
/// ](https://doc.rust-lang.org/reference/macros.html#macro-invocation).
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-const-definition"}}}
pub const X_MACRO: Uuid = uuid!["bb49a0cc-8490-5129-bec2-d536b43ba6a1"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XMacro;

impl XMacro {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        X_MACRO
    }
}

impl Default for XMacro {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
