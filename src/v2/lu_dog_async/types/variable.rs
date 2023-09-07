// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_async::types::local_variable::LocalVariable;
use crate::v2::lu_dog_async::types::parameter::Parameter;
use crate::v2::lu_dog_async::types::x_value::XValue;
use crate::v2::lu_dog_async::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-documentation"}}}
/// A Variable
///
/// A variable in a function. It may be either a local variable or a parameter.
///
/// A variable has a name, and and indirectly, via [`Value`], a type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Variable {
    pub subtype: VariableEnum,
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum VariableEnum {
    LambdaParameter(usize),
    LocalVariable(usize),
    Parameter(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl Variable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_lambda_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub async fn new_lambda_parameter(
        name: String,
        subtype: &Arc<RwLock<LambdaParameter>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Variable>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_variable(|id| {
                Arc::new(RwLock::new(Variable {
                    name: name.to_owned(),
                    subtype: VariableEnum::LambdaParameter(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_local_variable"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub async fn new_local_variable(
        name: String,
        subtype: &Arc<RwLock<LocalVariable>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Variable>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_variable(|id| {
                Arc::new(RwLock::new(Variable {
                    name: name.to_owned(),
                    subtype: VariableEnum::LocalVariable(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub async fn new_parameter(
        name: String,
        subtype: &Arc<RwLock<Parameter>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Variable>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_variable(|id| {
                Arc::new(RwLock::new(Variable {
                    name: name.to_owned(),
                    subtype: VariableEnum::Parameter(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub async fn r11_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r11_x_value");
        store
            .iter_x_value()
            .await
            .filter_map(|x_value| async move {
                if let XValueEnum::Variable(id) = x_value.read().await.subtype {
                    Some(x_value.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
