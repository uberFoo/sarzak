// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog::types::local_variable::LocalVariable;
use crate::v2::lu_dog::types::parameter::Parameter;
use crate::v2::lu_dog::types::x_value::XValue;
use crate::v2::lu_dog::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Variable {
    pub subtype: VariableEnum,
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VariableEnum {
    LambdaParameter(Uuid),
    LocalVariable(Uuid),
    Parameter(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl Variable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_lambda_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_lambda_parameter(
        name: String,
        subtype: &Rc<RefCell<LambdaParameter>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Variable>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Variable {
            name: name,
            subtype: VariableEnum::LambdaParameter(subtype.borrow().id), // b
            id,
        }));
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_local_variable"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_local_variable(
        name: String,
        subtype: &Rc<RefCell<LocalVariable>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Variable>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Variable {
            name: name,
            subtype: VariableEnum::LocalVariable(subtype.borrow().id), // b
            id,
        }));
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_parameter(
        name: String,
        subtype: &Rc<RefCell<Parameter>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Variable>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Variable {
            name: name,
            subtype: VariableEnum::Parameter(subtype.borrow().id), // b
            id,
        }));
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XValue>>> {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
