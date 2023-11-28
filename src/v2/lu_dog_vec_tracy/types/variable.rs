// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_vec_tracy::types::local_variable::LocalVariable;
use crate::v2::lu_dog_vec_tracy::types::parameter::Parameter;
use crate::v2::lu_dog_vec_tracy::types::x_value::XValue;
use crate::v2::lu_dog_vec_tracy::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
    pub fn new_lambda_parameter(
        name: String,
        subtype: &Rc<RefCell<LambdaParameter>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Variable>> {
        store.inter_variable(|id| {
            Rc::new(RefCell::new(Variable {
                name: name.to_owned(),
                subtype: VariableEnum::LambdaParameter(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_local_variable"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_local_variable(
        name: String,
        subtype: &Rc<RefCell<LocalVariable>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Variable>> {
        store.inter_variable(|id| {
            Rc::new(RefCell::new(Variable {
                name: name.to_owned(),
                subtype: VariableEnum::LocalVariable(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_parameter(
        name: String,
        subtype: &Rc<RefCell<Parameter>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Variable>> {
        store.inter_variable(|id| {
            Rc::new(RefCell::new(Variable {
                name: name.to_owned(),
                subtype: VariableEnum::Parameter(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r11_x_value");
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Variable(id) = x_value.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
