// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-use-statements"}}}
use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
use crate::v2::lu_dog_pl_mutex::types::false_literal::FALSE_LITERAL;
use crate::v2::lu_dog_pl_mutex::types::literal::Literal;
use crate::v2::lu_dog_pl_mutex::types::true_literal::TRUE_LITERAL;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-enum-documentation"}}}
/// A Boolean
///
/// It's either `true` or `false`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BooleanLiteral {
    FalseLiteral(Uuid),
    TrueLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-implementation"}}}
impl BooleanLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-new-impl"}}}
    /// Create a new instance of BooleanLiteral::FalseLiteral
    pub fn new_false_literal(store: &LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        // This is already in the store.
        store.exhume_boolean_literal(&FALSE_LITERAL).unwrap()
    }

    /// Create a new instance of BooleanLiteral::TrueLiteral
    pub fn new_true_literal(store: &LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        // This is already in the store.
        store.exhume_boolean_literal(&TRUE_LITERAL).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            BooleanLiteral::FalseLiteral(id) => *id,
            BooleanLiteral::TrueLiteral(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Literal>>> {
        span!("r22_literal");
        vec![store.exhume_literal(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}