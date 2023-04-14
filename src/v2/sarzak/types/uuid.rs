// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"uuid-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-struct-documentation"}}}
/// The UUID Type
///
/// I feel like there are too many implementation details here.
///
/// This UUID is expected to be version 5. Generally we produce input
/// to the hash function from other UUIDs, coupled with additional
/// information from the creator to ensure a unique UUID.
///
/// The `ns` attribute is the namespace used to generate generate UUIDs
/// given a particular instance of `UUID`.
///
/// ❗️{"singleton_object": true, "translation_name": "SarzakUuid"}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
pub const UUID: Uuid = uuid!["8d63fc44-28ea-599b-8654-877b133f0640"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
