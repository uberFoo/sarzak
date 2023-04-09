// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"some-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::woog_option::WoogOption;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-struct-documentation"}}}
/// Some Type
///
/// This type wraps another. It's used by the supertype, `[Option]`, to represent a type that
/// may or may not exist.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Some {
    pub id: Uuid,
    /// R2: [`Some`] 'contains' [`ValueType`]
    pub inner_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-implementation"}}}
impl Some {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-struct-impl-new"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub fn new(inner_type: &ValueType, store: &mut LuDogStore) -> Some {
        let id = Uuid::new_v4();
        let new = Some {
            id: id,
            inner_type: inner_type.id(),
        };
        store.inter_some(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-struct-impl-new_"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub fn new_(inner_type: &ValueType) -> Some {
        let id = Uuid::new_v4();
        let new = Some {
            id: id,
            inner_type: inner_type.id(),
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-struct-impl-nav-forward-to-inner_type"}}}
    /// Navigate to [`ValueType`] across R2(1-*)
    pub fn r2_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.inner_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"some-impl-nav-subtype-to-supertype-woog_option"}}}
    // Navigate to [`WoogOption`] across R3(isa)
    pub fn r3_woog_option<'a>(&'a self, store: &'a LuDogStore) -> Vec<&WoogOption> {
        vec![store.exhume_woog_option(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
