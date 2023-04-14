// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::float_literal::FLOAT_LITERAL;
use crate::v2::lu_dog::types::integer_literal::INTEGER_LITERAL;
use crate::v2::lu_dog::types::string_literal::STRING_LITERAL;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-enum-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Literal {
    BooleanLiteral(Uuid),
    FloatLiteral(Uuid),
    IntegerLiteral(Uuid),
    StringLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl Literal {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-new-impl"}}}
    /// Create a new instance of Literal::BooleanLiteral
    pub fn new_boolean_literal(boolean_literal: &BooleanLiteral, store: &mut LuDogStore) -> Self {
        let new = Self::BooleanLiteral(boolean_literal.id());
        store.inter_literal(new.clone());
        new
    }

    pub fn new_boolean_literal_(boolean_literal: &BooleanLiteral) -> Self {
        let new = Self::BooleanLiteral(boolean_literal.id());
        new
    }

    /// Create a new instance of Literal::FloatLiteral
    pub fn new_float_literal() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::FloatLiteral(FLOAT_LITERAL)
    }

    /// Create a new instance of Literal::IntegerLiteral
    pub fn new_integer_literal() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::IntegerLiteral(INTEGER_LITERAL)
    }

    /// Create a new instance of Literal::StringLiteral
    pub fn new_string_literal() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::StringLiteral(STRING_LITERAL)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Literal::BooleanLiteral(id) => *id,
            Literal::FloatLiteral(id) => *id,
            Literal::IntegerLiteral(id) => *id,
            Literal::StringLiteral(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
