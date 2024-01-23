// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_option-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::grace_type::GraceType;
use serde::{Deserialize, Serialize};

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
    pub fn new(ty: &Arc<RwLock<GraceType>>, store: &mut WoogStore) -> Arc<RwLock<WoogOption>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(WoogOption {
            id,
            ty: ty.read().unwrap().id(),
        }));
        store.inter_woog_option(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`GraceType`] across R20(1-*)
    pub fn r20_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<GraceType>>> {
        vec![store.exhume_grace_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-impl-nav-subtype-to-supertype-grace_type"}}}
    // Navigate to [`GraceType`] across R2(isa)
    pub fn r2_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<GraceType>>> {
        vec![store.exhume_grace_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
