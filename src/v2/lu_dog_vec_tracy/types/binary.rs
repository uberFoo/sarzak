// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::addition::ADDITION;
use crate::v2::lu_dog_vec_tracy::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_vec_tracy::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_vec_tracy::types::division::DIVISION;
use crate::v2::lu_dog_vec_tracy::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_vec_tracy::types::operator::Operator;
use crate::v2::lu_dog_vec_tracy::types::operator::OperatorEnum;
use crate::v2::lu_dog_vec_tracy::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-documentation"}}}
/// Binary Operators
///
/// +, -, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Binary {
    pub subtype: BinaryEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BinaryEnum {
    Addition(Uuid),
    Assignment(Uuid),
    BooleanOperator(usize),
    Division(Uuid),
    Multiplication(Uuid),
    Subtraction(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl Binary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_addition"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_addition(store: &mut LuDogVecTracyStore) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: BinaryEnum::Addition(ADDITION),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_assignment"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_assignment(store: &mut LuDogVecTracyStore) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: BinaryEnum::Assignment(ASSIGNMENT),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_boolean_operator"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_boolean_operator(
        subtype: &Rc<RefCell<BooleanOperator>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: BinaryEnum::BooleanOperator(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_division"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_division(store: &mut LuDogVecTracyStore) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: BinaryEnum::Division(DIVISION),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_multiplication"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_multiplication(store: &mut LuDogVecTracyStore) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: BinaryEnum::Multiplication(MULTIPLICATION),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_subtraction"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_subtraction(store: &mut LuDogVecTracyStore) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: BinaryEnum::Subtraction(SUBTRACTION),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r47_operator");
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
