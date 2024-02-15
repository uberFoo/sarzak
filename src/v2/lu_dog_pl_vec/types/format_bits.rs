// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_bits-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_pl_vec::types::format_string::FormatString;
use crate::v2::lu_dog_pl_vec::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatBits {
    pub subtype: FormatBitsEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FormatBitsEnum {
    ExpressionBit(usize),
    StringBit(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-implementation"}}}
impl FormatBits {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-new_expression_bit"}}}
    /// Inter a new FormatBits in the store, and return it's `id`.
    pub fn new_expression_bit(
        subtype: &Arc<RwLock<ExpressionBit>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<FormatBits>> {
        store.inter_format_bits(|id| {
            Arc::new(RwLock::new(FormatBits {
                subtype: FormatBitsEnum::ExpressionBit(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-new_string_bit"}}}
    /// Inter a new FormatBits in the store, and return it's `id`.
    pub fn new_string_bit(
        subtype: &Arc<RwLock<StringBit>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<FormatBits>> {
        store.inter_format_bits(|id| {
            Arc::new(RwLock::new(FormatBits {
                subtype: FormatBitsEnum::StringBit(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R112(1-1)
    pub fn r112_format_string<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.read().first_format_bit == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R111(1-1)
    pub fn r111_format_string<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.read().format_bits == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-implementation"}}}
impl PartialEq for FormatBits {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
