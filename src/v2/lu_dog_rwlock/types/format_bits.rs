// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_bits-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_rwlock::types::format_string::FormatString;
use crate::v2::lu_dog_rwlock::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FormatBits {
    pub subtype: FormatBitsEnum,
    pub id: Uuid,
    /// R111: [`FormatBits`] 'comprise' [`FormatString`]
    pub format_string: Uuid,
    /// R113: [`FormatBits`] 'next' [`FormatBits`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FormatBitsEnum {
    ExpressionBit(Uuid),
    StringBit(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-implementation"}}}
impl FormatBits {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-new_expression_bit"}}}
    /// Inter a new FormatBits in the store, and return it's `id`.
    pub fn new_expression_bit(
        format_string: &Arc<RwLock<FormatString>>,
        next: Option<&Arc<RwLock<FormatBits>>>,
        subtype: &Arc<RwLock<ExpressionBit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<FormatBits>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FormatBits {
            format_string: format_string.read().unwrap().id,
            next: next.map(|format_bits| format_bits.read().unwrap().id),
            subtype: FormatBitsEnum::ExpressionBit(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_format_bits(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-new_string_bit"}}}
    /// Inter a new FormatBits in the store, and return it's `id`.
    pub fn new_string_bit(
        format_string: &Arc<RwLock<FormatString>>,
        next: Option<&Arc<RwLock<FormatBits>>>,
        subtype: &Arc<RwLock<StringBit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<FormatBits>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FormatBits {
            format_string: format_string.read().unwrap().id,
            next: next.map(|format_bits| format_bits.read().unwrap().id),
            subtype: FormatBitsEnum::StringBit(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_format_bits(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-forward-to-format_string"}}}
    /// Navigate to [`FormatString`] across R111(1-*)
    pub fn r111_format_string<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store.exhume_format_string(&self.format_string).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FormatBits`] across R113(1-*c)
    pub fn r113_format_bits<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBits>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_format_bits(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-bi-cond-to-format_bits"}}}
    /// Navigate to [`FormatBits`] across R113(1c-1c)
    pub fn r113c_format_bits<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBits>>> {
        let format_bits = store
            .iter_format_bits()
            .find(|format_bits| format_bits.read().unwrap().next == Some(self.id));
        match format_bits {
            Some(ref format_bits) => vec![format_bits.clone()],
            None => Vec::new(),
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
            .find(|format_string| format_string.read().unwrap().first_format_bit == Some(self.id))
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
