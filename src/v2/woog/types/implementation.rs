// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-const-documentation"}}}
/// Dynamic Object Association
///
/// This scopes functions and constants to a [`Structure`] or an [`Enumeration`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-const-definition"}}}
pub const IMPLEMENTATION: Uuid = uuid!["d9423bdc-1217-5acc-9083-e13adccd9258"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Implementation;

impl Implementation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        IMPLEMENTATION
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
