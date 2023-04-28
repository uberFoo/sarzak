// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"conditional-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-documentation"}}}
/// A constant value that indicates a conditionality of _conditional_.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-impl-nav-subtype-to-supertype-conditionality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-impl-nav-subtype-to-supertype-conditionality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditional-impl-nav-subtype-to-supertype-conditionality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
pub const CONDITIONAL: Uuid = uuid!["1ef6f1f8-de66-552b-8d4a-a04215c37c1e"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Conditional;

impl Conditional {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        CONDITIONAL
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
