// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"empty-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"empty-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"empty-const-documentation"}}}
/// The Empty Type
///
/// This type represents the lack of a type. It's actually sort of a werid construct, because
///  it also implies the lack of a value. How can you have a value without a type?
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"empty-const-definition"}}}
pub const EMPTY: Uuid = uuid!["ea81f072-96fe-5426-9bc3-c0419e448fe5"];

pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        EMPTY
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
