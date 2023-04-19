// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unknown_variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unknown_variable-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unknown_variable-const-documentation"}}}
/// Unknown Variable
///
/// A variable was (de)referenced, and it was not found.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unknown_variable-const-definition"}}}
pub const UNKNOWN_VARIABLE: Uuid = uuid!["e8216f8f-db62-541a-89fc-a4fdc4434e96"];

pub struct UnknownVariable;

impl UnknownVariable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        UNKNOWN_VARIABLE
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
