// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"float-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-documentation"}}}
/// The Floating Point Type
///
/// This type holds numbers from ℝ. This type is just a placeholder. It's implementation is
///  determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
pub const FLOAT: Uuid = uuid!["1fe11221-b8be-5f99-a161-e378f90b094d"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Float;

impl Float {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        FLOAT
    }
}

impl Default for Float {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
