// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::addition::ADDITION;
use crate::v2::lu_dog_vanilla::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_vanilla::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_vanilla::types::division::DIVISION;
use crate::v2::lu_dog_vanilla::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_vanilla::types::operator::Operator;
use crate::v2::lu_dog_vanilla::types::operator::OperatorEnum;
use crate::v2::lu_dog_vanilla::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-documentation"}}}
/// Binary Operators
///
/// +, -, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Binary {
    pub subtype: BinaryEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BinaryEnum {
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_addition"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_addition(bogus: bool, store: &mut LuDogVanillaStore) -> Binary {
        let id = Uuid::new_v4();
        let new = Binary {
            bogus: bogus,
            subtype: BinaryEnum::Addition(ADDITION),
            id,
        };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_assignment"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_assignment(bogus: bool, store: &mut LuDogVanillaStore) -> Binary {
        let id = Uuid::new_v4();
        let new = Binary {
            bogus: bogus,
            subtype: BinaryEnum::Assignment(ASSIGNMENT),
            id,
        };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_boolean_operator"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_boolean_operator(
        bogus: bool,
        subtype: &BooleanOperator,
        store: &mut LuDogVanillaStore,
    ) -> Binary {
        let id = Uuid::new_v4();
        let new = Binary {
            bogus: bogus,
            subtype: BinaryEnum::BooleanOperator(subtype.id),
            id,
        };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_division"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_division(bogus: bool, store: &mut LuDogVanillaStore) -> Binary {
        let id = Uuid::new_v4();
        let new = Binary {
            bogus: bogus,
            subtype: BinaryEnum::Division(DIVISION),
            id,
        };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_multiplication"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_multiplication(bogus: bool, store: &mut LuDogVanillaStore) -> Binary {
        let id = Uuid::new_v4();
        let new = Binary {
            bogus: bogus,
            subtype: BinaryEnum::Multiplication(MULTIPLICATION),
            id,
        };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_subtraction"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_subtraction(bogus: bool, store: &mut LuDogVanillaStore) -> Binary {
        let id = Uuid::new_v4();
        let new = Binary {
            bogus: bogus,
            subtype: BinaryEnum::Subtraction(SUBTRACTION),
            id,
        };
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Operator> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.subtype {
                    id == self.id
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
