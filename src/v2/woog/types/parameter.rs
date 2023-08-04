// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::woog::types::function::Function;
use crate::v2::woog::types::variable::Variable;
use crate::v2::woog::types::variable::VariableEnum;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-documentation"}}}
/// Parameter
///
/// A parameter is an input to a function.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Parameter {
    pub id: Uuid,
    pub seed: Uuid,
    /// R5: [`Parameter`] 'is used by a' [`Function`]
    pub function: Option<Uuid>,
    /// R1: [`Parameter`] 'came before' [`Parameter`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new 'Parameter' in the store, and return it's `id`.
    pub fn new(
        seed: Uuid,
        function: Option<&Rc<RefCell<Function>>>,
        next: Option<&Rc<RefCell<Parameter>>>,
        store: &mut WoogStore,
    ) -> Rc<RefCell<Parameter>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Parameter {
            id,
            seed,
            function: function.map(|function| function.borrow().id),
            next: next.map(|parameter| parameter.borrow().id),
        }));
        store.inter_parameter(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-function"}}}
    /// Navigate to [`Function`] across R5(1-*c)
    pub fn r5_function<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r5_function");
        match self.function {
            Some(ref function) => vec![store.exhume_function(&function).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R1(1-*c)
    pub fn r1_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Parameter>>> {
        span!("r1_parameter");
        match self.next {
            Some(ref next) => vec![store.exhume_parameter(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R1(1c-1c)
    pub fn r1c_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Parameter>>> {
        span!("r1_parameter");
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.borrow().next == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R8(isa)
    pub fn r8_variable<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<Variable>>> {
        span!("r8_variable");
        vec![store
            .iter_variable()
            .find(|variable| {
                if let VariableEnum::Parameter(id) = variable.borrow().subtype {
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
