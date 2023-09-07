// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"lambda_parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::lambda::Lambda;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::variable::Variable;
use crate::v2::lu_dog_async::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-documentation"}}}
/// id
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LambdaParameter {
    pub id: usize,
    pub position: i64,
    /// R76: [`LambdaParameter`] 'helps define a function signature' [`Lambda`]
    pub lambda: usize,
    /// R75: [`LambdaParameter`] '' [`LambdaParameter`]
    pub next: Option<usize>,
    /// R77: [`LambdaParameter`] 'may require a type' [`ValueType`]
    pub ty: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-implementation"}}}
impl LambdaParameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-new"}}}
    /// Inter a new 'Lambda Parameter' in the store, and return it's `id`.
    pub async fn new(
        position: i64,
        lambda: &Arc<RwLock<Lambda>>,
        next: Option<&Arc<RwLock<LambdaParameter>>>,
        ty: Option<&Arc<RwLock<ValueType>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<LambdaParameter>> {
        let lambda_parameter = match next {
            Some(lambda_parameter) => Some(lambda_parameter.read().await.id),
            None => None,
        };
        let value_type = match ty {
            Some(value_type) => Some(value_type.read().await.id),
            None => None,
        };
        let lambda = lambda.read().await.id;
        store
            .inter_lambda_parameter(|id| {
                Arc::new(RwLock::new(LambdaParameter {
                    id,
                    position,
                    lambda,
                    next: lambda_parameter,
                    ty: value_type,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-to-lambda"}}}
    /// Navigate to [`Lambda`] across R76(1-*)
    pub async fn r76_lambda<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Lambda>>> {
        span!("r76_lambda");
        vec![store.exhume_lambda(&self.lambda).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`LambdaParameter`] across R75(1-*c)
    pub async fn r75_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        span!("r75_lambda_parameter");
        match self.next {
            Some(ref next) => vec![store.exhume_lambda_parameter(next).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R77(1-*c)
    pub async fn r77_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r77_value_type");
        match self.ty {
            Some(ref ty) => vec![store.exhume_value_type(ty).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-backward-one-bi-cond-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R75(1c-1c)
    pub async fn r75c_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        span!("r75_lambda_parameter");
        store
            .iter_lambda_parameter()
            .await
            .filter_map(|lambda_parameter| async move {
                if lambda_parameter.read().await.next == Some(self.id) {
                    Some(lambda_parameter.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub async fn r12_variable<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Variable>>> {
        span!("r12_variable");
        store
            .iter_variable()
            .await
            .filter_map(|variable| async move {
                if let VariableEnum::LambdaParameter(id) = variable.read().await.subtype {
                    Some(variable.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-implementation"}}}
impl PartialEq for LambdaParameter {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.lambda == other.lambda
            && self.next == other.next
            && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
