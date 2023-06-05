// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"top-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"top-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"top-const-documentation"}}}
/// The top edge of the rendered box
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"top-const-definition"}}}
pub const TOP: Uuid = uuid!["e9a50304-bcda-5842-8fd3-329876e838c2"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Top;

impl Top {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        TOP
    }
}

impl Default for Top {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
