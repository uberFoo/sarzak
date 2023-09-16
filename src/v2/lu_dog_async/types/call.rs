// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::argument::Argument;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::function_call::FUNCTION_CALL;
use crate::v2::lu_dog_async::types::macro_call::MACRO_CALL;
use crate::v2::lu_dog_async::types::method_call::MethodCall;
use crate::v2::lu_dog_async::types::static_method_call::StaticMethodCall;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-hybrid-documentation"}}}
/// A Call, of some sort
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Call {
    pub subtype: CallEnum,
    pub arg_check: bool,
    pub id: usize,
    /// R81: [`Call`] 'may have a first' [`Argument`]
    pub argument: Option<usize>,
    /// R29: [`Call`] 'may be an' [`Expression`]
    pub expression: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CallEnum {
    FunctionCall(Uuid),
    MacroCall(Uuid),
    MethodCall(usize),
    StaticMethodCall(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-implementation"}}}
impl Call {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_function_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_function_call(
        arg_check: bool,
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let argument = match argument {
            Some(argument) => Some(argument.read().await.id),
            None => None,
        };
        let expression = match expression {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_call(|id| {
                Arc::new(RwLock::new(Call {
                    arg_check: arg_check,
                    argument,   // (a)
                    expression, // (a)
                    subtype: CallEnum::FunctionCall(FUNCTION_CALL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_macro_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_macro_call(
        arg_check: bool,
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let argument = match argument {
            Some(argument) => Some(argument.read().await.id),
            None => None,
        };
        let expression = match expression {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_call(|id| {
                Arc::new(RwLock::new(Call {
                    arg_check: arg_check,
                    argument,   // (a)
                    expression, // (a)
                    subtype: CallEnum::MacroCall(MACRO_CALL),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_method_call(
        arg_check: bool,
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<MethodCall>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let s_id = subtype.read().await.id;
        let argument = match argument {
            Some(argument) => Some(argument.read().await.id),
            None => None,
        };
        let expression = match expression {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_call(|id| {
                Arc::new(RwLock::new(Call {
                    arg_check: arg_check,
                    argument,   // (a)
                    expression, // (a)
                    subtype: CallEnum::MethodCall(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_static_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_static_method_call(
        arg_check: bool,
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<StaticMethodCall>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let s_id = subtype.read().await.id;
        let argument = match argument {
            Some(argument) => Some(argument.read().await.id),
            None => None,
        };
        let expression = match expression {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_call(|id| {
                Arc::new(RwLock::new(Call {
                    arg_check: arg_check,
                    argument,   // (a)
                    expression, // (a)
                    subtype: CallEnum::StaticMethodCall(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R81(1-*c)
    pub async fn r81_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Argument>>> + '_ {
        span!("r81_argument");
        match self.argument {
            Some(ref argument) => {
                stream::iter(vec![store.exhume_argument(argument).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R29(1-*c)
    pub async fn r29_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        span!("r29_expression");
        match self.expression {
            Some(ref expression) => {
                stream::iter(vec![store.exhume_expression(expression).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub async fn r28_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Argument>>> + '_ {
        span!("r28_argument");
        store.iter_argument().await.filter_map(|argument| async {
            if argument.read().await.function == self.id {
                Some(argument)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::Call(id) = expression.read().await.subtype {
                    Some(expression.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-implementation"}}}
impl PartialEq for Call {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.arg_check == other.arg_check
            && self.argument == other.argument
            && self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
