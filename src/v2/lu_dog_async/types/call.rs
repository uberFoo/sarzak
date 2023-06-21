// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::argument::Argument;
use crate::v2::lu_dog_async::types::expression::Expression;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Call {
    pub subtype: CallEnum,
    pub arg_check: bool,
    pub id: Uuid,
    /// R29: [`Call`] 'may be an' [`Expression`]
    pub expression: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CallEnum {
    FunctionCall(Uuid),
    MacroCall(Uuid),
    MethodCall(Uuid),
    StaticMethodCall(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-implementation"}}}
impl Call {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_function_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_function_call(
        arg_check: bool,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            expression: futures::future::OptionFuture::from(
                expression.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: CallEnum::FunctionCall(FUNCTION_CALL),
            id,
        }));
        store.inter_call(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_macro_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_macro_call(
        arg_check: bool,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            expression: futures::future::OptionFuture::from(
                expression.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: CallEnum::MacroCall(MACRO_CALL),
            id,
        }));
        store.inter_call(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_method_call(
        arg_check: bool,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<MethodCall>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            expression: futures::future::OptionFuture::from(
                expression.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: CallEnum::MethodCall(subtype.read().await.id),
            id,
        }));
        store.inter_call(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_static_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub async fn new_static_method_call(
        arg_check: bool,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<StaticMethodCall>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            expression: futures::future::OptionFuture::from(
                expression.map(|expression| async { expression.read().await.id() }),
            )
            .await,
            subtype: CallEnum::StaticMethodCall(subtype.read().await.id),
            id,
        }));
        store.inter_call(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R29(1-*c)
    pub async fn r29_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r29_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(expression).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub async fn r28_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Argument>>> {
        span!("r28_argument");
        let mut result = Vec::new();
        for argument in store.iter_argument().await {
            if argument.read().await.function == self.id {
                result.push(argument)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
