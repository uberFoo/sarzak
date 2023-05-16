// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"range-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-hybrid-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-const-definition"}}}
pub const RANGE: Uuid = uuid!["50e24d8f-3739-5fd9-b5ca-f537eba4b21a"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-hybrid-enum-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-implementation"}}}
pub struct Range;

impl Range {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-new_from"}}}
    pub fn new() -> Self {
        Self {}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-new_full"}}}

    pub fn id(&self) -> Uuid {
        RANGE
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-new_inclusive"}}}
}

impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-new_to"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-new_to_inclusive"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-nav-forward-cond-to-lhs"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-struct-impl-nav-forward-cond-to-rhs"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"range-impl-nav-subtype-to-supertype-expression"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
