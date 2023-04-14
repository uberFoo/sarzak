// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"integer-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-documentation"}}}
/// The Integer Type
///
/// This is an interger that can hold positive and negative values. This type is just a placeholder
///. It's implementation is determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-struct-impl-new_"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer-impl-nav-subtype-to-supertype-ty"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
pub const INTEGER: Uuid = uuid!["045f5d22-f71b-5845-9113-b8b859d1dbac"];

pub struct Integer;

impl Integer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        INTEGER
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
