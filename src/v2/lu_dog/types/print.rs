// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"print-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-const-documentation"}}}
/// A Print Expression?
///
/// Shold this be a statement?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-const-definition"}}}
pub const PRINT: Uuid = uuid!["f7ec5e60-3685-5e1f-ac0a-292ab1849d64"];

pub struct Print;

impl Print {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        PRINT
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
