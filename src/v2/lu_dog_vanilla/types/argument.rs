// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"argument-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::call::Call;
use crate::v2::lu_dog_vanilla::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-documentation"}}}
/// An Argument to a Function Call
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Argument {
    pub id: Uuid,
    pub position: i64,
    /// R37: [`Argument`] '' [`Expression`]
    pub expression: Uuid,
    /// R28: [`Argument`] 'is part of a' [`Call`]
    pub function: Uuid,
    /// R27: [`Argument`] 'follows' [`Argument`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-implementation"}}}
impl Argument {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-new"}}}
    /// Inter a new 'Argument' in the store, and return it's `id`.
    pub fn new(
        position: i64,
        expression: &Expression,
        function: &Call,
        next: Option<&Argument>,
        store: &mut LuDogVanillaStore,
    ) -> Argument {
        let id = Uuid::new_v4();
        let new = Argument {
            id,
            position,
            expression: expression.id,
            function: function.id,
            next: next.as_ref().map(|argument| argument.id),
        };
        store.inter_argument(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R37(1-*)
    pub fn r37_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-function"}}}
    /// Navigate to [`Call`] across R28(1-*)
    pub fn r28_call<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Call> {
        vec![store.exhume_call(&self.function).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Argument`] across R27(1-*c)
    pub fn r27_argument<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Argument> {
        match self.next {
            Some(ref next) => vec![store.exhume_argument(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R27(1c-1c)
    pub fn r27c_argument<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Argument> {
        let argument = store
            .iter_argument()
            .find(|argument| argument.next == Some(self.id));
        match argument {
            Some(ref argument) => vec![argument],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-call"}}}
    /// Navigate to [`Call`] across R81(1c-1c)
    pub fn r81c_call<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Call> {
        let call = store
            .iter_call()
            .find(|call| call.argument == Some(self.id));
        match call {
            Some(ref call) => vec![call],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
