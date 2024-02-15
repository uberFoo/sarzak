// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_string-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::format_bits::FormatBits;
use crate::v2::lu_dog_rwlock::types::literal::Literal;
use crate::v2::lu_dog_rwlock::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FormatString {
    pub id: Uuid,
    /// R112: [`FormatString`] 'needs to first' [`FormatBits`]
    pub first_format_bit: Option<Uuid>,
    /// R111: [`FormatString`] 'is comprised of' [`FormatBits`]
    pub format_bits: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-implementation"}}}
impl FormatString {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-new"}}}
    /// Inter a new 'Format String' in the store, and return it's `id`.
    pub fn new(
        first_format_bit: Option<&Arc<RwLock<FormatBits>>>,
        format_bits: Option<&Arc<RwLock<FormatBits>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<FormatString>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(FormatString {
            id,
            first_format_bit: first_format_bit.map(|format_bits| format_bits.read().unwrap().id()),
            format_bits: format_bits.map(|format_bits| format_bits.read().unwrap().id()),
        }));
        store.inter_format_string(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-list"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-first_format_bit"}}}
    /// Navigate to [`FormatBits`] across R112(1-*c)
    pub fn r112_format_bits<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBits>>> {
        match self.first_format_bit {
            Some(ref first_format_bit) => {
                vec![store.exhume_format_bits(&first_format_bit).unwrap()]
            }
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-format_bits"}}}
    /// Navigate to [`FormatBits`] across R111(1-*c)
    pub fn r111_format_bits<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FormatBits>>> {
        match self.format_bits {
            Some(ref format_bits) => vec![store.exhume_format_bits(&format_bits).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Literal>>> {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
