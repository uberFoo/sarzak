// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
use crate::v2::lu_dog_rwlock::types::addition::ADDITION;
use crate::v2::lu_dog_rwlock::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_rwlock::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_rwlock::types::division::DIVISION;
use crate::v2::lu_dog_rwlock::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_rwlock::types::operator::Operator;
use crate::v2::lu_dog_rwlock::types::operator::OperatorEnum;
use crate::v2::lu_dog_rwlock::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
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
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn new_addition(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&ADDITION).unwrap()
    }

    /// Create a new instance of Binary::Assignment
    pub fn new_assignment(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&ASSIGNMENT).unwrap()
    }

    /// Create a new instance of Binary::BooleanOperator
    pub fn new_boolean_operator(
        boolean_operator: &Arc<RwLock<BooleanOperator>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = boolean_operator.read().unwrap().id();
        if let Some(boolean_operator) = store.exhume_binary(&id) {
            boolean_operator
        } else {
            let new = Arc::new(RwLock::new(Self::BooleanOperator(id)));
            store.inter_binary(new.clone());
            new
        }
    }

    /// Create a new instance of Binary::Division
    pub fn new_division(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&DIVISION).unwrap()
    }

    /// Create a new instance of Binary::Multiplication
    pub fn new_multiplication(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&MULTIPLICATION).unwrap()
    }

    /// Create a new instance of Binary::Subtraction
    pub fn new_subtraction(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&SUBTRACTION).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Addition(id) => *id,
            Self::Assignment(id) => *id,
            Self::BooleanOperator(id) => *id,
            Self::Division(id) => *id,
            Self::Multiplication(id) => *id,
            Self::Subtraction(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.read().unwrap().subtype {
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
