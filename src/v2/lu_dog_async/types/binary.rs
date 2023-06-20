// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::addition::ADDITION;
use crate::v2::lu_dog_async::types::assignment::ASSIGNMENT;
use crate::v2::lu_dog_async::types::boolean_operator::BooleanOperator;
use crate::v2::lu_dog_async::types::division::DIVISION;
use crate::v2::lu_dog_async::types::multiplication::MULTIPLICATION;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::operator::OperatorEnum;
use crate::v2::lu_dog_async::types::subtraction::SUBTRACTION;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
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
    pub async fn new_addition(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&ADDITION).await.unwrap()
    }

    /// Create a new instance of Binary::Assignment
    pub async fn new_assignment(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&ASSIGNMENT).await.unwrap()
    }

    /// Create a new instance of Binary::BooleanOperator
    pub async fn new_boolean_operator(
        boolean_operator: &Arc<RwLock<BooleanOperator>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = boolean_operator.read().await.id();
        if let Some(boolean_operator) = store.exhume_binary(&id).await {
            boolean_operator
        } else {
            let new = Arc::new(RwLock::new(Self::BooleanOperator(id)));
            store.inter_binary(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Binary::Division
    pub async fn new_division(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&DIVISION).await.unwrap()
    }

    /// Create a new instance of Binary::Multiplication
    pub async fn new_multiplication(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&MULTIPLICATION).await.unwrap()
    }

    /// Create a new instance of Binary::Subtraction
    pub async fn new_subtraction(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_binary(&SUBTRACTION).await.unwrap()
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
    pub async fn r47_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        let mut result = Vec::new();
        for operator in store.iter_operator().await {
            if let OperatorEnum::Binary(id) = operator.read().await.subtype {
                result.push(operator.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
