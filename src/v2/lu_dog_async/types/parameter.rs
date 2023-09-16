// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::variable::Variable;
use crate::v2::lu_dog_async::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-documentation"}}}
/// A Parameter to a Function
///
/// From inside the function it's a parameter, from outside it's an argument. No idea why I
///  wrote that — just looking for content...  I mean, what else do you say about a parameter
/// ?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameter {
    pub id: usize,
    pub position: i64,
    /// R13: [`Parameter`] 'is available to a' [`Function`]
    pub function: usize,
    /// R14: [`Parameter`] 'follows' [`Parameter`]
    pub next: Option<usize>,
    /// R79: [`Parameter`] 'requires a' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new 'Parameter' in the store, and return it's `id`.
    pub async fn new(
        position: i64,
        function: &Arc<RwLock<Function>>,
        next: Option<&Arc<RwLock<Parameter>>>,
        ty: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Parameter>> {
        let function = function.read().await.id;
        let parameter = match next {
            Some(parameter) => Some(parameter.read().await.id),
            None => None,
        };
        let ty = ty.read().await.id;
        store
            .inter_parameter(|id| {
                Arc::new(RwLock::new(Parameter {
                    id,
                    position,
                    function,
                    next: parameter,
                    ty,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-function"}}}
    /// Navigate to [`Function`] across R13(1-*)
    pub async fn r13_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Function>>> + '_ {
        span!("r13_function");
        stream::iter(vec![store.exhume_function(&self.function).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R14(1-*c)
    pub async fn r14_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Parameter>>> + '_ {
        span!("r14_parameter");
        match self.next {
            Some(ref next) => {
                stream::iter(vec![store.exhume_parameter(next).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R79(1-*)
    pub async fn r79_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ValueType>>> + '_ {
        span!("r79_value_type");
        stream::iter(vec![store.exhume_value_type(&self.ty).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-function"}}}
    /// Navigate to [`Function`] across R82(1c-1c)
    pub async fn r82c_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Function>>> + '_ {
        span!("r82_function");
        store
            .iter_function()
            .await
            .filter_map(move |function| async move {
                if function.read().await.first_param == Some(self.id) {
                    Some(function.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R14(1c-1c)
    pub async fn r14c_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Parameter>>> + '_ {
        span!("r14_parameter");
        store
            .iter_parameter()
            .await
            .filter_map(move |parameter| async move {
                if parameter.read().await.next == Some(self.id) {
                    Some(parameter.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-impl-nav-subtype-to-supertype-variable"}}}
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
                if let VariableEnum::Parameter(id) = variable.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.function == other.function
            && self.next == other.next
            && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
