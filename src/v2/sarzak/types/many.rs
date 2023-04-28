// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"many-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-documentation"}}}
/// A constant value that indicates a cardinality of _many_.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-impl-nav-subtype-to-supertype-cardinality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-impl-nav-subtype-to-supertype-cardinality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"many-impl-nav-subtype-to-supertype-cardinality"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
pub const MANY: Uuid = uuid!["481a6ff0-caee-5817-8812-857f58f7c215"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Many;

impl Many {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        MANY
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
