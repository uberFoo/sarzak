// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"shared-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"shared-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"shared-const-documentation"}}}
/// A shared borrow.
///
/// According to rust rules, you may have any number of shared references outstanding at one
/// time. Just as long as there are zero mutable references.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"shared-const-definition"}}}
pub const SHARED: Uuid = uuid!["87082daa-8c8e-5745-be20-331f2ca86caf"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
