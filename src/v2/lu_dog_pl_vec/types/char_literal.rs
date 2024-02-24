// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"char_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::literal::Literal;
use crate::v2::lu_dog_pl_vec::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharLiteral {
    pub id: usize,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-implementation"}}}
impl CharLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-struct-impl-new"}}}
    /// Inter a new 'Char Literal' in the store, and return it's `id`.
    pub fn new(x_value: i64, store: &mut LuDogPlVecStore) -> Arc<RwLock<CharLiteral>> {
        store.inter_char_literal(|id| Arc::new(RwLock::new(CharLiteral { id, x_value })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Literal>>> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::CharLiteral(id) = literal.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-implementation"}}}
impl PartialEq for CharLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
