// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"lambda-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::body::Body;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-documentation"}}}
/// Lambda Function
///
/// It’s a function, it has a type, parameters, etc. It does not have a name, which is problematic
///  with Function having one. It’s also an Expression, unlike a Function.
///
/// I should think about creating another function subtype that contains just the name...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lambda {
    pub id: usize,
    /// R73: [`Lambda`] 'contains a' [`Body`]
    pub body: Option<usize>,
    /// R103: [`Lambda`] 'may have a' [`LambdaParameter`]
    pub first_param: Option<usize>,
    /// R74: [`Lambda`] 'has a' [`ValueType`]
    pub return_type: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-implementation"}}}
impl Lambda {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-new"}}}
    /// Inter a new 'Lambda' in the store, and return it's `id`.
    pub async fn new(
        body: Option<&Arc<RwLock<Body>>>,
        first_param: Option<&Arc<RwLock<LambdaParameter>>>,
        return_type: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Lambda>> {
        let return_type = return_type.read().await.id;
        let lambda_parameter = match first_param {
            Some(lambda_parameter) => Some(lambda_parameter.read().await.id),
            None => None,
        };
        let body = match body {
            Some(body) => Some(body.read().await.id),
            None => None,
        };
        store
            .inter_lambda(|id| {
                Arc::new(RwLock::new(Lambda {
                    id,
                    body,
                    first_param: lambda_parameter,
                    return_type,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-body"}}}
    /// Navigate to [`Body`] across R73(1-*c)
    pub async fn r73_body<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Body>>> + '_ {
        match self.body {
            Some(ref body) => {
                stream::iter(vec![store.exhume_body(body).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-first_param"}}}
    /// Navigate to [`LambdaParameter`] across R103(1-*c)
    pub async fn r103_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        match self.first_param {
            Some(ref first_param) => stream::iter(
                vec![store.exhume_lambda_parameter(first_param).await.unwrap()].into_iter(),
            ),
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R74(1-*)
    pub async fn r74_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ValueType>>> + '_ {
        stream::iter(vec![store.exhume_value_type(&self.return_type).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-backward-1_M-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R76(1-M)
    pub async fn r76_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        store
            .iter_lambda_parameter()
            .await
            .filter_map(|lambda_parameter| async {
                if lambda_parameter.read().await.lambda == self.id {
                    Some(lambda_parameter)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::Lambda(id) = expression.read().await.subtype {
                    Some(expression.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub async fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        store
            .iter_value_type()
            .await
            .filter_map(|value_type| async move {
                if let ValueTypeEnum::Lambda(id) = value_type.read().await.subtype {
                    Some(value_type.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-implementation"}}}
impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
            && self.first_param == other.first_param
            && self.return_type == other.return_type
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
