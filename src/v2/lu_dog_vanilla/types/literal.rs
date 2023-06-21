// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
use crate::v2::lu_dog_vanilla::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_vanilla::types::expression::Expression;
use crate::v2::lu_dog_vanilla::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_vanilla::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_vanilla::types::string_literal::StringLiteral;
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
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn new_boolean_literal(
        boolean_literal: &BooleanLiteral,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::BooleanLiteral(boolean_literal.id());
        store.inter_literal(new.clone());
        new
    }

    /// Create a new instance of Literal::FloatLiteral
    pub fn new_float_literal(float_literal: &FloatLiteral, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::FloatLiteral(float_literal.id);
        store.inter_literal(new.clone());
        new
    }

    /// Create a new instance of Literal::IntegerLiteral
    pub fn new_integer_literal(
        integer_literal: &IntegerLiteral,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::IntegerLiteral(integer_literal.id);
        store.inter_literal(new.clone());
        new
    }

    /// Create a new instance of Literal::StringLiteral
    pub fn new_string_literal(
        string_literal: &StringLiteral,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::StringLiteral(string_literal.id);
        store.inter_literal(new.clone());
        new
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
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
