// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
use crate::v2::lu_dog_vanilla::types::addition::ADDITION;
use crate::v2::lu_dog_vanilla::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_vanilla::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_vanilla::types::division::DIVISION;
use crate::v2::lu_dog_vanilla::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_vanilla::types::operator::Operator;
use crate::v2::lu_dog_vanilla::types::operator::OperatorEnum;
use crate::v2::lu_dog_vanilla::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-enum-documentation"}}}
/// Binary Operators
///
/// +, -, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Binary {
    Addition(Uuid),
    Assignment(Uuid),
    BooleanOperator(Uuid),
    Division(Uuid),
    Multiplication(Uuid),
    Subtraction(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl Binary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-new-impl"}}}
    /// Create a new instance of Binary::Addition
    pub fn new_addition() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Addition(ADDITION)
    }

    /// Create a new instance of Binary::Assignment
    pub fn new_assignment() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Assignment(ASSIGNMENT)
    }

    /// Create a new instance of Binary::BooleanOperator
    pub fn new_boolean_operator(
        boolean_operator: &BooleanOperator,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::BooleanOperator(boolean_operator.id());
        store.inter_binary(new.clone());
        new
    }

    /// Create a new instance of Binary::Division
    pub fn new_division() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Division(DIVISION)
    }

    /// Create a new instance of Binary::Multiplication
    pub fn new_multiplication() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Multiplication(MULTIPLICATION)
    }

    /// Create a new instance of Binary::Subtraction
    pub fn new_subtraction() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Subtraction(SUBTRACTION)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Binary::Addition(id) => *id,
            Binary::Assignment(id) => *id,
            Binary::BooleanOperator(id) => *id,
            Binary::Division(id) => *id,
            Binary::Multiplication(id) => *id,
            Binary::Subtraction(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Operator> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
