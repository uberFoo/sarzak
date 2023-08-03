// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"lambda_parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::lambda::Lambda;
use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::variable::Variable;
use crate::v2::lu_dog::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-documentation"}}}
/// id
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LambdaParameter {
    pub id: Uuid,
    pub position: i64,
    /// R76: [`LambdaParameter`] 'helps define a function signature' [`Lambda`]
    pub lambda: Uuid,
    /// R75: [`LambdaParameter`] '' [`LambdaParameter`]
    pub next: Option<Uuid>,
    /// R77: [`LambdaParameter`] 'may require a type' [`ValueType`]
    pub ty: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-implementation"}}}
impl LambdaParameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-new"}}}
    /// Inter a new 'Lambda Parameter' in the store, and return it's `id`.
    pub fn new(
        position: i64,
        lambda: &Rc<RefCell<Lambda>>,
        next: Option<&Rc<RefCell<LambdaParameter>>>,
        ty: Option<&Rc<RefCell<ValueType>>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<LambdaParameter>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(LambdaParameter {
            id,
            position,
            lambda: lambda.borrow().id,
            next: next.map(|lambda_parameter| lambda_parameter.borrow().id),
            ty: ty.map(|value_type| value_type.borrow().id()),
        }));
        store.inter_lambda_parameter(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-to-lambda"}}}
    /// Navigate to [`Lambda`] across R76(1-*)
    pub fn r76_lambda<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Lambda>>> {
        span!("r76_lambda");
        vec![store.exhume_lambda(&self.lambda).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`LambdaParameter`] across R75(1-*c)
    pub fn r75_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        span!("r75_lambda_parameter");
        match self.next {
            Some(ref next) => vec![store.exhume_lambda_parameter(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R77(1-*c)
    pub fn r77_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
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
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        span!("r75_lambda_parameter");
        let lambda_parameter = store
            .iter_lambda_parameter()
            .find(|lambda_parameter| lambda_parameter.borrow().next == Some(self.id));
        match lambda_parameter {
            Some(ref lambda_parameter) => vec![lambda_parameter.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda_parameter-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub fn r12_variable<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Variable>>> {
        span!("r12_variable");
        vec![store
            .iter_variable()
            .find(|variable| {
                if let VariableEnum::LambdaParameter(id) = variable.borrow().subtype {
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
