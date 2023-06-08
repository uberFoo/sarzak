// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"negation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-const-documentation"}}}
/// The unary minus
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-const-definition"}}}
pub const NEGATION: Uuid = uuid!["a4671940-9194-5585-84b4-4bd22b975f6f"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-implementation"}}}
pub struct Negation;

impl Negation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-impl-new"}}}
    pub fn new() -> Self {
        Self {}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-impl-nav-forward-to-expr"}}}

    pub fn id(&self) -> Uuid {
        NEGATION
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-impl-nav-subtype-to-supertype-expression"}}}
}

impl Default for Negation {
    fn default() -> Self {
        Self::new()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
