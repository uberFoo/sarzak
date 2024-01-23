// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::addition::ADDITION;
use crate::v2::lu_dog_async::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_async::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_async::types::division::DIVISION;
use crate::v2::lu_dog_async::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::operator::OperatorEnum;
use crate::v2::lu_dog_async::types::subtraction::SUBTRACTION;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    pub bogus: bool,
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
    pub async fn new_addition(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<Binary>> {
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    bogus: bogus,
                    subtype: BinaryEnum::Addition(ADDITION),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_assignment"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub async fn new_assignment(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<Binary>> {
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    bogus: bogus,
                    subtype: BinaryEnum::Assignment(ASSIGNMENT),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_boolean_operator"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub async fn new_boolean_operator(
        bogus: bool,
        subtype: &Arc<RwLock<BooleanOperator>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Binary>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    bogus: bogus,
                    subtype: BinaryEnum::BooleanOperator(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_division"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub async fn new_division(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<Binary>> {
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    bogus: bogus,
                    subtype: BinaryEnum::Division(DIVISION),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_multiplication"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub async fn new_multiplication(
        bogus: bool,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Binary>> {
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    bogus: bogus,
                    subtype: BinaryEnum::Multiplication(MULTIPLICATION),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new_subtraction"}}}
    /// Inter a new Binary in the store, and return it's `id`.
    pub async fn new_subtraction(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<Binary>> {
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    bogus: bogus,
                    subtype: BinaryEnum::Subtraction(SUBTRACTION),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub async fn r47_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        store
            .iter_operator()
            .await
            .filter_map(|operator| async move {
                if let OperatorEnum::Binary(id) = operator.read().await.subtype {
                    Some(operator.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
