// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::argument::Argument;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock::types::function_call::FunctionCall;
use crate::v2::lu_dog_rwlock::types::macro_call::MACRO_CALL;
use crate::v2::lu_dog_rwlock::types::method_call::MethodCall;
use crate::v2::lu_dog_rwlock::types::static_method_call::StaticMethodCall;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<FunctionCall>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.read().unwrap().id),
            expression: expression.map(|expression| expression.read().unwrap().id),
            subtype: CallEnum::FunctionCall(subtype.read().unwrap().id), // b
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
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.read().unwrap().id),
            expression: expression.map(|expression| expression.read().unwrap().id),
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
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<MethodCall>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.read().unwrap().id),
            expression: expression.map(|expression| expression.read().unwrap().id),
            subtype: CallEnum::MethodCall(subtype.read().unwrap().id), // b
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
        argument: Option<&Arc<RwLock<Argument>>>,
        expression: Option<&Arc<RwLock<Expression>>>,
        subtype: &Arc<RwLock<StaticMethodCall>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Call>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Call {
            arg_check: arg_check,
            argument: argument.map(|argument| argument.read().unwrap().id),
            expression: expression.map(|expression| expression.read().unwrap().id),
            subtype: CallEnum::StaticMethodCall(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R81(1-*c)
    pub fn r81_argument<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Argument>>> {
        match self.argument {
            Some(ref argument) => vec![store.exhume_argument(&argument).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R29(1-*c)
    pub fn r29_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(&expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub fn r28_argument<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Argument>>> {
        store
            .iter_argument()
            .filter(|argument| argument.read().unwrap().function == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Call(id) = expression.read().unwrap().subtype {
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
