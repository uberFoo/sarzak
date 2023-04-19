// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unknown-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unknown-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unknown-const-documentation"}}}
/// Unknown Type
///
/// The type is unknown.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unknown-const-definition"}}}
pub const UNKNOWN: Uuid = uuid!["351f1018-eae0-5fdb-920b-0528c8947138"];

pub struct Unknown;

impl Unknown {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        UNKNOWN
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
