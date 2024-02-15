// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_bits-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-use-statements"}}}
use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
use crate::v2::lu_dog_rwlock::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_rwlock::types::format_string::FormatString;
use crate::v2::lu_dog_rwlock::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FormatBits {
    ExpressionBit(Uuid),
    StringBit(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-implementation"}}}
impl FormatBits {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-new-impl"}}}
    /// Create a new instance of FormatBits::ExpressionBit
    pub fn new_expression_bit(
        expression_bit: &Arc<RwLock<ExpressionBit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = expression_bit.read().unwrap().id;
        if let Some(expression_bit) = store.exhume_format_bits(&id) {
            expression_bit
        } else {
            let new = Arc::new(RwLock::new(Self::ExpressionBit(id)));
            store.inter_format_bits(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of FormatBits::StringBit
    pub fn new_string_bit(
        string_bit: &Arc<RwLock<StringBit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = string_bit.read().unwrap().id;
        if let Some(string_bit) = store.exhume_format_bits(&id) {
            string_bit
        } else {
            let new = Arc::new(RwLock::new(Self::StringBit(id)));
            store.inter_format_bits(new.clone());
            new
        }
    } // wtf?

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::ExpressionBit(id) => *id,
            Self::StringBit(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R112(1-1)
    pub fn r112_format_string<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.read().unwrap().first_format_bit == Some(self.id()))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R111(1-1)
    pub fn r111_format_string<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.read().unwrap().format_bits == Some(self.id()))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
