// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"float_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::literal::Literal;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-struct-documentation"}}}
/// A Floating Point Literal
///
/// Nothing fancy. No scientific notation.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FloatLiteral {
    pub id: Uuid,
    pub x_value: f64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-implementation"}}}
impl FloatLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-struct-impl-new"}}}
    /// Inter a new 'Float Literal' in the store, and return it's `id`.
    pub fn new(x_value: f64, store: &mut LuDogRwlockStore) -> Arc<RwLock<FloatLiteral>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FloatLiteral { id, x_value }));
        store.inter_float_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Literal>>> {
        span!("r22_literal");
        vec![store.exhume_literal(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
