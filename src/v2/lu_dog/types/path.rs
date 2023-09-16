// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"path-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path-const-documentation"}}}
/// This is a path to a local variable, or an item. It is made up of scopes, separated by `
/// ::`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path-const-definition"}}}
pub const PATH: Uuid = uuid!["9a08ad53-69fd-5651-9417-e344bb5805a5"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Path;

impl Path {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        PATH
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
