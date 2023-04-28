// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"one-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-documentation"}}}
/// A constant value that indicates a cardinality of _one_.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-impl-nav-subtype-to-supertype-cardinality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-impl-nav-subtype-to-supertype-cardinality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-impl-nav-subtype-to-supertype-cardinality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
pub const ONE: Uuid = uuid!["84d60cb6-04cf-5c82-9e38-79d3999b5d5c"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct One;

impl One {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        ONE
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
