// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::function_call::FUNCTION_CALL;
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
    pub id: Uuid,
    /// R29: [`Call`] 'may be an' [`Expression`]
    pub expression: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CallEnum {
    FunctionCall(Uuid),
    MethodCall(Uuid),
    StaticMethodCall(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-implementation"}}}
impl Call {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_function_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub fn new_function_call(
        expression: Option<&Arc<Mutex<Expression>>>,
        store: &mut LuDogStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
            expression: expression.map(|expression| expression.lock().id()),
            subtype: CallEnum::FunctionCall(FUNCTION_CALL),
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new_method_call"}}}
    /// Inter a new Call in the store, and return it's `id`.
    pub fn new_method_call(
        expression: Option<&Arc<Mutex<Expression>>>,
        subtype: &Arc<Mutex<MethodCall>>,
        store: &mut LuDogStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
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
        expression: Option<&Arc<Mutex<Expression>>>,
        subtype: &Arc<Mutex<StaticMethodCall>>,
        store: &mut LuDogStore,
    ) -> Arc<Mutex<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Call {
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
    pub fn r29_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Expression>>> {
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub fn r28_argument<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Argument>>> {
        store
            .iter_argument()
            .filter(|argument| argument.lock().function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Expression>>> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
