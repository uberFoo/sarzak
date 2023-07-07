// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"lambda_parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::lambda::Lambda;
use crate::v2::lu_dog_rwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_rwlock_vec::types::variable::Variable;
use crate::v2::lu_dog_rwlock_vec::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-documentation"}}}
/// id
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LambdaParameter {
    pub id: usize,
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
    pub fn new(
        lambda: &Arc<RwLock<Lambda>>,
        next: Option<&Arc<RwLock<LambdaParameter>>>,
        ty: Option<&Arc<RwLock<ValueType>>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<LambdaParameter>> {
        store.inter_lambda_parameter(|id| {
            Arc::new(RwLock::new(LambdaParameter {
                id,
                lambda: lambda.read().unwrap().id,
                next: next.map(|lambda_parameter| lambda_parameter.read().unwrap().id),
                ty: ty.map(|value_type| value_type.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-to-lambda"}}}
    /// Navigate to [`Lambda`] across R76(1-*)
    pub fn r76_lambda<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Lambda>>> {
        span!("r76_lambda");
        vec![store.exhume_lambda(&self.lambda).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`LambdaParameter`] across R75(1-*c)
    pub fn r75_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        span!("r75_lambda_parameter");
        match self.next {
            Some(ref next) => vec![store.exhume_lambda_parameter(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R77(1-*c)
    pub fn r77_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r77_value_type");
        match self.ty {
            Some(ref ty) => vec![store.exhume_value_type(&ty).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-backward-one-bi-cond-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R75(1c-1c)
    pub fn r75c_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        span!("r75_lambda_parameter");
        let lambda_parameter = store
            .iter_lambda_parameter()
            .find(|lambda_parameter| lambda_parameter.read().unwrap().next == Some(self.id));
        match lambda_parameter {
            Some(ref lambda_parameter) => vec![lambda_parameter.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub fn r12_variable<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Variable>>> {
        span!("r12_variable");
        vec![store
            .iter_variable()
            .find(|variable| {
                if let VariableEnum::LambdaParameter(id) = variable.read().unwrap().subtype {
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
