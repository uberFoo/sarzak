// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"owned-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"owned-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"owned-const-documentation"}}}
/// An owned value
///
/// Someone has to be responsible for it, right?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"owned-const-definition"}}}
pub const OWNED: Uuid = uuid!["81f5c36f-be98-5d4e-9896-26a8b13653cd"];

pub struct Owned;

impl Owned {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        OWNED
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
