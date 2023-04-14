// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::method_call::MethodCall;
use crate::v2::lu_dog::types::static_method_call::StaticMethodCall;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-hybrid-documentation"}}}
/// A Function Call
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionCall {
    pub subtype: FunctionCallEnum,
    pub id: Uuid,
    /// R29: [`FunctionCall`] 'may be an' [`Expression`]
    pub expression: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FunctionCallEnum {
    MethodCall(Uuid),
    StaticMethodCall(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-implementation"}}}
impl FunctionCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-new_method_call"}}}
    /// Inter a new FunctionCall in the store, and return it's `id`.
    pub fn new_method_call(
        expression: Option<&Expression>,
        subtype: &MethodCall,
        store: &mut LuDogStore,
    ) -> FunctionCall {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = FunctionCall {
            expression: expression.map(|expression| expression.id()),
            subtype: FunctionCallEnum::MethodCall(subtype.id),
            id,
        };
        store.inter_function_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-new_method_call_"}}}
    /// Inter a new FunctionCall in the store, and return it's `id`.
    pub fn new_method_call_(expression: Option<&Expression>, subtype: &MethodCall) -> FunctionCall {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = FunctionCall {
            expression: expression.map(|expression| expression.id()),
            subtype: FunctionCallEnum::MethodCall(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-new_static_method_call"}}}
    /// Inter a new FunctionCall in the store, and return it's `id`.
    pub fn new_static_method_call(
        expression: Option<&Expression>,
        subtype: &StaticMethodCall,
        store: &mut LuDogStore,
    ) -> FunctionCall {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = FunctionCall {
            expression: expression.map(|expression| expression.id()),
            subtype: FunctionCallEnum::StaticMethodCall(subtype.id),
            id,
        };
        store.inter_function_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-new_static_method_call_"}}}
    /// Inter a new FunctionCall in the store, and return it's `id`.
    pub fn new_static_method_call_(
        expression: Option<&Expression>,
        subtype: &StaticMethodCall,
    ) -> FunctionCall {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = FunctionCall {
            expression: expression.map(|expression| expression.id()),
            subtype: FunctionCallEnum::StaticMethodCall(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R29(1-*c)
    pub fn r29_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R28(1-M)
    pub fn r28_argument<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Argument> {
        store
            .iter_argument()
            .filter_map(|argument| {
                if argument.function == self.id {
                    Some(argument)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function_call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
