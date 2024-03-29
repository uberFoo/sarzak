// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"public-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"public-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"public-const-documentation"}}}
/// Public Visibility
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"public-const-definition"}}}
pub const PUBLIC: Uuid = uuid!["9600c3e7-157a-5757-8a05-0eff604b9ba5"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Public;

impl Public {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        PUBLIC
    }
}

impl Default for Public {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
