// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"s_string-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-const-documentation"}}}
/// The String Type
///
/// This type holds unicode characters. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_string-const-definition"}}}
pub const S_STRING: Uuid = uuid!["eb75208b-bf2e-55f9-9a38-7c1553f31960"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

pub struct SString;

impl SString {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        S_STRING
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
