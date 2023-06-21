// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::argument::Argument;
use crate::v2::lu_dog_pl_mutex::types::expression::Expression;
use crate::v2::lu_dog_pl_mutex::types::function_call::FUNCTION_CALL;
use crate::v2::lu_dog_pl_mutex::types::macro_call::MACRO_CALL;
use crate::v2::lu_dog_pl_mutex::types::method_call::MethodCall;
use crate::v2::lu_dog_pl_mutex::types::static_method_call::StaticMethodCall;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
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
        expression: Option<&Arc<Mutex<Expression>>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
            arg_check: arg_check,
            expression: expression.map(|expression| expression.lock().id()),
            subtype: CallEnum::FunctionCall(FUNCTION_CALL),
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
        expression: Option<&Arc<Mutex<Expression>>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
            arg_check: arg_check,
            expression: expression.map(|expression| expression.lock().id()),
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
        expression: Option<&Arc<Mutex<Expression>>>,
        subtype: &Arc<Mutex<MethodCall>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
            arg_check: arg_check,
            expression: expression.map(|expression| expression.lock().id()),
            subtype: CallEnum::MethodCall(subtype.lock().id),
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
        expression: Option<&Arc<Mutex<Expression>>>,
        subtype: &Arc<Mutex<StaticMethodCall>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
            arg_check: arg_check,
            expression: expression.map(|expression| expression.lock().id()),
            subtype: CallEnum::StaticMethodCall(subtype.lock().id),
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R29(1-*c)
    pub fn r29_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r29_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub fn r28_argument<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Argument>>> {
        span!("r28_argument");
        store
            .iter_argument()
            .filter(|argument| argument.lock().function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
