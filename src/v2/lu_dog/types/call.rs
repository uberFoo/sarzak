// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::expression::ExpressionEnum;
use crate::v2::lu_dog::types::function_call::FunctionCall;
use crate::v2::lu_dog::types::macro_call::MACRO_CALL;
use crate::v2::lu_dog::types::method_call::MethodCall;
use crate::v2::lu_dog::types::static_method_call::StaticMethodCall;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
    /// R81: [`Call`] 'may have a first' [`Argument`]
    pub argument: Option<Uuid>,
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
    pub fn new_function_call(
        arg_check: bool,
        argument: Option<&Rc<RefCell<Argument>>>,
        expression: Option<&Rc<RefCell<Expression>>>,
        subtype: &Rc<RefCell<FunctionCall>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Call>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.borrow().id),
            expression: expression.map(|expression| expression.borrow().id),
            subtype: CallEnum::FunctionCall(subtype.borrow().id), // b
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_macro_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub fn new_macro_call(
        arg_check: bool,
        argument: Option<&Rc<RefCell<Argument>>>,
        expression: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Call>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.borrow().id),
            expression: expression.map(|expression| expression.borrow().id),
            subtype: CallEnum::MacroCall(MACRO_CALL),
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub fn new_method_call(
        arg_check: bool,
        argument: Option<&Rc<RefCell<Argument>>>,
        expression: Option<&Rc<RefCell<Expression>>>,
        subtype: &Rc<RefCell<MethodCall>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Call>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.borrow().id),
            expression: expression.map(|expression| expression.borrow().id),
            subtype: CallEnum::MethodCall(subtype.borrow().id), // b
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_static_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub fn new_static_method_call(
        arg_check: bool,
        argument: Option<&Rc<RefCell<Argument>>>,
        expression: Option<&Rc<RefCell<Expression>>>,
        subtype: &Rc<RefCell<StaticMethodCall>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Call>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.borrow().id),
            expression: expression.map(|expression| expression.borrow().id),
            subtype: CallEnum::StaticMethodCall(subtype.borrow().id), // b
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R81(1-*c)
    pub fn r81_argument<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Argument>>> {
        match self.argument {
            Some(ref argument) => vec![store.exhume_argument(&argument).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R29(1-*c)
    pub fn r29_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(&expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub fn r28_argument<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Argument>>> {
        store
            .iter_argument()
            .filter(|argument| argument.borrow().function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Call(id) = expression.borrow().subtype {
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
