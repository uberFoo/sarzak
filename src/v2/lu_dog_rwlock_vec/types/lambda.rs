// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"lambda-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::body::Body;
use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock_vec::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_rwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_rwlock_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-documentation"}}}
/// Lambda Function
///
/// It’s a function, it has a type, parameters, etc. It does not have a name, which is problematic
///  with Function having one. It’s also an Expression, unlike a Function.
///
/// I should think about creating another function subtype that contains just the name...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-const-definition"}}}
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
    pub fn new(
        body: Option<&Arc<RwLock<Body>>>,
        first_param: Option<&Arc<RwLock<LambdaParameter>>>,
        return_type: &Arc<RwLock<ValueType>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Lambda>> {
        store.inter_lambda(|id| {
            Arc::new(RwLock::new(Lambda {
                id,
                body: body.map(|body| body.read().unwrap().id),
                first_param: first_param
                    .map(|lambda_parameter| lambda_parameter.read().unwrap().id),
                return_type: return_type.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-to-body"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-body"}}}
    /// Navigate to [`Body`] across R73(1-*c)
    pub fn r73_body<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Body>>> {
        match self.body {
            Some(ref body) => vec![store.exhume_body(&body).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-first_param"}}}
    /// Navigate to [`LambdaParameter`] across R103(1-*c)
    pub fn r103_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        match self.first_param {
            Some(ref first_param) => vec![store.exhume_lambda_parameter(&first_param).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R74(1-*)
    pub fn r74_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store.exhume_value_type(&self.return_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-backward-1_M-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R76(1-M)
    pub fn r76_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.read().unwrap().lambda == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Lambda(id) = expression.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Lambda(id) = value_type.read().unwrap().subtype {
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
