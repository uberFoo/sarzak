// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"argument-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::call::Call;
use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-documentation"}}}
/// An Argument to a Function Call
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Argument {
    pub id: usize,
    pub position: i64,
    /// R37: [`Argument`] '' [`Expression`]
    pub expression: usize,
    /// R28: [`Argument`] 'is part of a' [`Call`]
    pub function: usize,
    /// R27: [`Argument`] 'follows' [`Argument`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-implementation"}}}
impl Argument {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-new"}}}
    /// Inter a new 'Argument' in the store, and return it's `id`.
    pub fn new(
        position: i64,
        expression: &Arc<RwLock<Expression>>,
        function: &Arc<RwLock<Call>>,
        next: Option<&Arc<RwLock<Argument>>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Argument>> {
        store.inter_argument(|id| {
            Arc::new(RwLock::new(Argument {
                id,
                position,
                expression: expression.read().unwrap().id,
                function: function.read().unwrap().id,
                next: next.map(|argument| argument.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R37(1-*)
    pub fn r37_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-function"}}}
    /// Navigate to [`Call`] across R28(1-*)
    pub fn r28_call<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Call>>> {
        vec![store.exhume_call(&self.function).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Argument`] across R27(1-*c)
    pub fn r27_argument<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Argument>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_argument(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R27(1c-1c)
    pub fn r27c_argument<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Argument>>> {
        let argument = store
            .iter_argument()
            .find(|argument| argument.read().unwrap().next == Some(self.id));
        match argument {
            Some(ref argument) => vec![argument.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-call"}}}
    /// Navigate to [`Call`] across R81(1c-1c)
    pub fn r81c_call<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Call>>> {
        let call = store
            .iter_call()
            .find(|call| call.read().unwrap().argument == Some(self.id));
        match call {
            Some(ref call) => vec![call.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-implementation"}}}
impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.expression == other.expression
            && self.function == other.function
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
