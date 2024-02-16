// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_string-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::format_bit::FormatBit;
use crate::v2::lu_dog_ndrwlock_vec::types::literal::Literal;
use crate::v2::lu_dog_ndrwlock_vec::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatString {
    pub id: usize,
    /// R112: [`FormatString`] 'needs to first' [`FormatBit`]
    pub first_format_bit: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-implementation"}}}
impl FormatString {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-new"}}}
    /// Inter a new 'Format String' in the store, and return it's `id`.
    pub fn new(
        first_format_bit: Option<&Arc<RwLock<FormatBit>>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<FormatString>> {
        store.inter_format_string(|id| {
            Arc::new(RwLock::new(FormatString {
                id,
                first_format_bit: first_format_bit.map(|format_bit| format_bit.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-list"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-first_format_bit"}}}
    /// Navigate to [`FormatBit`] across R112(1-*c)
    pub fn r112_format_bit<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<FormatBit>>> {
        match self.first_format_bit {
            Some(ref first_format_bit) => vec![store.exhume_format_bit(&first_format_bit).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-format_bits"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-backward-1_M-to-format_bits"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-backward-1_M-to-format_bit"}}}
    /// Navigate to [`FormatBit`] across R111(1-M)
    pub fn r111_format_bit<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<FormatBit>>> {
        store
            .iter_format_bit()
            .filter(|format_bit| format_bit.read().unwrap().format_string == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Literal>>> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::FormatString(id) = literal.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-implementation"}}}
impl PartialEq for FormatString {
    fn eq(&self, other: &Self) -> bool {
        self.first_format_bit == other.first_format_bit
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
