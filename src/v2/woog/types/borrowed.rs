// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"borrowed-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-const-documentation"}}}
/// Borrowed
///
/// The type is declared as borrowed.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-const-definition"}}}
pub const BORROWED: Uuid = uuid!["4ccd6011-dbef-5503-9015-e5c139895e54"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Borrowed;

impl Borrowed {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        BORROWED
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
