// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"integer_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::literal::Literal;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-struct-documentation"}}}
/// An Integer
///
/// I'm not sure what to do about width. I think I coded it as an i64 in the parser.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IntegerLiteral {
    pub id: Uuid,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-implementation"}}}
impl IntegerLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-struct-impl-new"}}}
    /// Inter a new 'Integer Literal' in the store, and return it's `id`.
    pub async fn new(x_value: i64, store: &mut LuDogAsyncStore) -> Arc<RwLock<IntegerLiteral>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(IntegerLiteral { id, x_value }));
        store.inter_integer_literal(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub async fn r22_literal<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Literal>>> {
        span!("r22_literal");
        vec![store.exhume_literal(&self.id).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
