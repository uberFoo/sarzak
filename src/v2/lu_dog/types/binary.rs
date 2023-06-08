// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::addition::ADDITION;
use crate::v2::lu_dog::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog::types::division::DIVISION;
use crate::v2::lu_dog::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::operator::OperatorEnum;
use crate::v2::lu_dog::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
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
    pub fn new_addition(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_binary(&ADDITION).unwrap()
    }

    /// Create a new instance of Binary::Assignment
    pub fn new_assignment(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_binary(&ASSIGNMENT).unwrap()
    }

    /// Create a new instance of Binary::BooleanOperator
    pub fn new_boolean_operator(
        boolean_operator: &Rc<RefCell<BooleanOperator>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = boolean_operator.borrow().id();
        if let Some(boolean_operator) = store.exhume_binary(&id) {
            boolean_operator
        } else {
            let new = Rc::new(RefCell::new(Self::BooleanOperator(id)));
            store.inter_binary(new.clone());
            new
        }
    }

    /// Create a new instance of Binary::Division
    pub fn new_division(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_binary(&DIVISION).unwrap()
    }

    /// Create a new instance of Binary::Multiplication
    pub fn new_multiplication(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_binary(&MULTIPLICATION).unwrap()
    }

    /// Create a new instance of Binary::Subtraction
    pub fn new_subtraction(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_binary(&SUBTRACTION).unwrap()
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
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r47_operator");
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.borrow().subtype {
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
