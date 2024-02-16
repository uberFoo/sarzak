// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression_bit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::format_bits::FormatBits;
use crate::v2::lu_dog_rwlock::types::format_bits::FormatBitsEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExpressionBit {
    pub id: Uuid,
    /// R109: [`ExpressionBit`] 'refers to an' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-implementation"}}}
impl ExpressionBit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-struct-impl-new"}}}
    /// Inter a new 'Expression Bit' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<ExpressionBit>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(ExpressionBit {
            id,
            expression: expression.read().unwrap().id,
        }));
        store.inter_expression_bit(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R109(1-*)
    pub fn r109_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-impl-nav-subtype-to-supertype-format_bits"}}}
    // Navigate to [`FormatBits`] across R110(isa)
    pub fn r110_format_bits<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBits>>> {
        vec![store
            .iter_format_bits()
            .find(|format_bits| {
                if let FormatBitsEnum::ExpressionBit(id) = format_bits.read().unwrap().subtype {
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
