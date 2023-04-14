// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"right-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"right-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"right-const-documentation"}}}
/// The right side of a rendered box
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"right-const-definition"}}}
pub const RIGHT: Uuid = uuid!["c824949b-058d-5145-981c-4c91a6554d96"];

pub struct Right;

impl Right {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        RIGHT
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
