// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"argument-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::call::Call;
use crate::v2::lu_dog_async::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-documentation"}}}
/// An Argument to a Function Call
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Argument {
    pub id: usize,
    pub position: i64,
    /// R37: [`Argument`] '' [`Expression`]
    pub expression: usize,
    /// R28: [`Argument`] 'is part of a' [`Call`]
    pub function: usize,
    /// R27: [`Argument`] 'follows' [`Argument`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-implementation"}}}
impl Argument {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-new"}}}
    /// Inter a new 'Argument' in the store, and return it's `id`.
    pub async fn new(
        position: i64,
        expression: &Arc<RwLock<Expression>>,
        function: &Arc<RwLock<Call>>,
        next: Option<&Arc<RwLock<Argument>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Argument>> {
        let expression = expression.read().await.id;
        let function = function.read().await.id;
        let argument = match next {
            Some(argument) => Some(argument.read().await.id),
            None => None,
        };
        store
            .inter_argument(|id| {
                Arc::new(RwLock::new(Argument {
                    id,
                    position,
                    expression,
                    function,
                    next: argument,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R37(1-*)
    pub async fn r37_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r37_expression");
        vec![store.exhume_expression(&self.expression).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-function"}}}
    /// Navigate to [`Call`] across R28(1-*)
    pub async fn r28_call<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Call>>> {
        span!("r28_call");
        vec![store.exhume_call(&self.function).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Argument`] across R27(1-*c)
    pub async fn r27_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Argument>>> {
        span!("r27_argument");
        match self.next {
            Some(ref next) => vec![store.exhume_argument(next).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R27(1c-1c)
    pub async fn r27c_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Argument>>> {
        span!("r27_argument");
        store
            .iter_argument()
            .await
            .filter_map(|argument| async move {
                if argument.read().await.next == Some(self.id) {
                    Some(argument.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-call"}}}
    /// Navigate to [`Call`] across R81(1c-1c)
    pub async fn r81c_call<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Call>>> {
        span!("r81_call");
        store
            .iter_call()
            .await
            .filter_map(|call| async move {
                if call.read().await.argument == Some(self.id) {
                    Some(call.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-implementation"}}}
impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.expression == other.expression
            && self.function == other.function
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
