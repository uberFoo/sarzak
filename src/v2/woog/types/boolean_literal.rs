// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::false_literal::FALSE_LITERAL;
use crate::v2::woog::types::literal::Literal;
use crate::v2::woog::types::true_literal::TRUE_LITERAL;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-enum-documentation"}}}
/// A Boolean Literal
///
/// Either `true` or `false`. Easy-peasy.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BooleanLiteral {
    FalseLiteral(Uuid),
    TrueLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-implementation"}}}
impl BooleanLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-new-impl"}}}
    /// Create a new instance of BooleanLiteral::FalseLiteral
    pub fn new_false_literal() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::FalseLiteral(FALSE_LITERAL)
    }

    /// Create a new instance of BooleanLiteral::TrueLiteral
    pub fn new_true_literal() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::TrueLiteral(TRUE_LITERAL)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            BooleanLiteral::FalseLiteral(id) => *id,
            BooleanLiteral::TrueLiteral(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R32(isa)
    pub fn r32_literal<'a>(&'a self, store: &'a WoogStore) -> Vec<&Literal> {
        vec![store.exhume_literal(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
