// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::addition::ADDITION;
use crate::v2::lu_dog_rwlock::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_rwlock::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_rwlock::types::division::DIVISION;
use crate::v2::lu_dog_rwlock::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_rwlock::types::operator::Operator;
use crate::v2::lu_dog_rwlock::types::operator::OperatorEnum;
use crate::v2::lu_dog_rwlock::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-hybrid-documentation"}}}
/// Binary Operators
///
/// +, -, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-enum-definition"}}}
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_addition"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_addition(bogus: bool, store: &mut LuDogRwlockStore) -> Arc<RwLock<Binary>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Binary {
            bogus: bogus,
            subtype: BinaryEnum::Addition(ADDITION),
            id,
        }));
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_assignment"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_assignment(bogus: bool, store: &mut LuDogRwlockStore) -> Arc<RwLock<Binary>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Binary {
            bogus: bogus,
            subtype: BinaryEnum::Assignment(ASSIGNMENT),
            id,
        }));
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_boolean_operator"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_boolean_operator(
        bogus: bool,
        subtype: &Arc<RwLock<BooleanOperator>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Binary>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Binary {
            bogus: bogus,
            subtype: BinaryEnum::BooleanOperator(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_division"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_division(bogus: bool, store: &mut LuDogRwlockStore) -> Arc<RwLock<Binary>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Binary {
            bogus: bogus,
            subtype: BinaryEnum::Division(DIVISION),
            id,
        }));
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_multiplication"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_multiplication(bogus: bool, store: &mut LuDogRwlockStore) -> Arc<RwLock<Binary>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Binary {
            bogus: bogus,
            subtype: BinaryEnum::Multiplication(MULTIPLICATION),
            id,
        }));
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_subtraction"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub fn new_subtraction(bogus: bool, store: &mut LuDogRwlockStore) -> Arc<RwLock<Binary>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Binary {
            bogus: bogus,
            subtype: BinaryEnum::Subtraction(SUBTRACTION),
            id,
        }));
        store.inter_binary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Operator>>> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Binary(id) = operator.read().unwrap().subtype {
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
