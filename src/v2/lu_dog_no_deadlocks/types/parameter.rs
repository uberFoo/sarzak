// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::function::Function;
use crate::v2::lu_dog_rwlock::types::variable::Variable;
use crate::v2::lu_dog_rwlock::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Parameter {
    pub id: Uuid,
    /// R13: [`Parameter`] 'is available to a' [`Function`]
    pub function: Uuid,
    /// R14: [`Parameter`] 'follows' [`Parameter`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new 'Parameter' in the store, and return it's `id`.
    pub fn new(
        function: &Arc<RwLock<Function>>,
        next: Option<&Arc<RwLock<Parameter>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Parameter>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Parameter {
            id,
            function: function.read().unwrap().id,
            next: next.map(|parameter| parameter.read().unwrap().id),
        }));
        store.inter_parameter(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-function"}}}
    /// Navigate to [`Function`] across R13(1-*)
    pub fn r13_function<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Function>>> {
        span!("r13_function");
        vec![store.exhume_function(&self.function).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R14(1-*c)
    pub fn r14_parameter<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r14_parameter");
        match self.next {
            Some(ref next) => vec![store.exhume_parameter(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R14(1c-1c)
    pub fn r14c_parameter<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r14_parameter");
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.read().unwrap().next == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R12(isa)
    pub fn r12_variable<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Variable>>> {
        span!("r12_variable");
        vec![store
            .iter_variable()
            .find(|variable| {
                if let VariableEnum::Parameter(id) = variable.read().unwrap().subtype {
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