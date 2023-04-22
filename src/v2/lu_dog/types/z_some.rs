// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_some-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::value::Value;
use crate::v2::lu_dog::types::woog_option::WoogOption;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-documentation"}}}
/// Some Type
///
/// This type wraps another. It's used by the supertype, `[Option]`, to represent a type that
///  may or may not exist.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ZSome {
    pub id: Uuid,
    /// R23: [`ZSome`] 'contains' [`Value`]
    pub inner: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-implementation"}}}
impl ZSome {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-new"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub fn new(inner: &Value, store: &mut LuDogStore) -> ZSome {
        let id = Uuid::new_v4();
        let new = ZSome {
            id: id,
            inner: inner.id,
        };
        store.inter_z_some(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-new_"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub fn new_(inner: &Value) -> ZSome {
        let id = Uuid::new_v4();
        let new = ZSome {
            id: id,
            inner: inner.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-nav-forward-to-inner"}}}
    /// Navigate to [`Value`] across R23(1-*)
    pub fn r23_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Value> {
        vec![store.exhume_value(&self.inner).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-woog_option"}}}
    // Navigate to [`WoogOption`] across R3(isa)
    pub fn r3_woog_option<'a>(&'a self, store: &'a LuDogStore) -> Vec<&WoogOption> {
        vec![store.exhume_woog_option(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
