// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_option-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-use-statements"}}}
use crate::v2::woog::types::grace_type::GraceType;
use crate::v2::woog::UUID_NS;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-documentation"}}}
/// An optional Type
///
/// In rust this is wrapped in an [`Option<T>`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WoogOption {
    pub id: Uuid,
    /// R20: [`WoogOption`] 'contains' [`GraceType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-implementation"}}}
impl WoogOption {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new"}}}
    /// Inter a new 'Option' in the store, and return it's `id`.
    pub fn new(ty: &GraceType, store: &mut WoogStore) -> WoogOption {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", ty).as_bytes());
        let new = WoogOption { ty: ty.id(), id };
        store.inter_woog_option(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`GraceType`] across R20(1-*)
    pub fn r20_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<&GraceType> {
        vec![store.exhume_grace_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-impl-nav-subtype-to-supertype-grace_type"}}}
    // Navigate to [`GraceType`] across R2(isa)
    pub fn r2_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<&GraceType> {
        vec![store.exhume_grace_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
