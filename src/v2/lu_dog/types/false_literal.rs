// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"false_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-struct-documentation"}}}
/// False Literal
///
/// The literal `false`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-struct-definition"}}}
pub const FALSE_LITERAL: Uuid = uuid!["a904e4be-d9ae-568d-9767-1098b31aba7f"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-implementation"}}}
pub struct FalseLiteral;

impl FalseLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-struct-impl-new"}}}
    pub fn new() -> Self {
        Self {}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"false_literal-impl-nav-subtype-to-supertype-boolean_literal"}}}

    pub fn id(&self) -> Uuid {
        FALSE_LITERAL
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}

impl Default for FalseLiteral {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
