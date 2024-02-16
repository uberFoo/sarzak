// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression_bit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::expression::Expression;
use crate::v2::lu_dog_pl_vec::types::format_bit::FormatBit;
use crate::v2::lu_dog_pl_vec::types::format_bit::FormatBitEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExpressionBit {
    pub id: usize,
    /// R109: [`ExpressionBit`] 'refers to an' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-implementation"}}}
impl ExpressionBit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-struct-impl-new"}}}
    /// Inter a new 'Expression Bit' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<ExpressionBit>> {
        store.inter_expression_bit(|id| {
            Arc::new(RwLock::new(ExpressionBit {
                id,
                expression: expression.read().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R109(1-*)
    pub fn r109_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-impl-nav-subtype-to-supertype-format_bits"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-impl-nav-subtype-to-supertype-format_bit"}}}
    // Navigate to [`FormatBit`] across R110(isa)
    pub fn r110_format_bit<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FormatBit>>> {
        vec![store
            .iter_format_bit()
            .find(|format_bit| {
                if let FormatBitEnum::ExpressionBit(id) = format_bit.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_bit-implementation"}}}
impl PartialEq for ExpressionBit {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
