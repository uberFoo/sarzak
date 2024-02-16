// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_bit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_rwlock::types::format_string::FormatString;
use crate::v2::lu_dog_rwlock::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FormatBit {
    pub subtype: FormatBitEnum,
    pub id: Uuid,
    /// R111: [`FormatBit`] 'comprise' [`FormatString`]
    pub format_string: Uuid,
    /// R113: [`FormatBit`] 'next' [`FormatBit`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FormatBitEnum {
    ExpressionBit(Uuid),
    StringBit(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-implementation"}}}
impl FormatBit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-new_expression_bit"}}}
    /// Inter a new FormatBit in the store, and return it's `id`.
    pub fn new_expression_bit(
        format_string: &Arc<RwLock<FormatString>>,
        next: Option<&Arc<RwLock<FormatBit>>>,
        subtype: &Arc<RwLock<ExpressionBit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<FormatBit>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FormatBit {
            format_string: format_string.read().unwrap().id,
            next: next.map(|format_bit| format_bit.read().unwrap().id),
            subtype: FormatBitEnum::ExpressionBit(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_format_bit(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-new_string_bit"}}}
    /// Inter a new FormatBit in the store, and return it's `id`.
    pub fn new_string_bit(
        format_string: &Arc<RwLock<FormatString>>,
        next: Option<&Arc<RwLock<FormatBit>>>,
        subtype: &Arc<RwLock<StringBit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<FormatBit>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FormatBit {
            format_string: format_string.read().unwrap().id,
            next: next.map(|format_bit| format_bit.read().unwrap().id),
            subtype: FormatBitEnum::StringBit(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_format_bit(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-forward-to-format_string"}}}
    /// Navigate to [`FormatString`] across R111(1-*)
    pub fn r111_format_string<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store.exhume_format_string(&self.format_string).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FormatBit`] across R113(1-*c)
    pub fn r113_format_bit<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBit>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_format_bit(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-backward-one-bi-cond-to-format_bit"}}}
    /// Navigate to [`FormatBit`] across R113(1c-1c)
    pub fn r113c_format_bit<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBit>>> {
        let format_bit = store
            .iter_format_bit()
            .find(|format_bit| format_bit.read().unwrap().next == Some(self.id));
        match format_bit {
            Some(ref format_bit) => vec![format_bit.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R112(1-1)
    pub fn r112_format_string<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatString>>> {
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.read().unwrap().first_format_bit == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
