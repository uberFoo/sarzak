// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::ownership::Ownership;
use crate::v2::woog::types::visibility::Visibility;
use crate::v2::woog::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-documentation"}}}
/// Value Access
///
/// This is what I'm calling the combined ideas of mutability and visibility.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Access {
    pub id: Uuid,
    /// R15: [`Access`] 'has' [`Ownership`]
    pub ownership: Uuid,
    /// R14: [`Access`] 'has' [`Visibility`]
    pub visibility: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-implementation"}}}
impl Access {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-new"}}}
    /// Inter a new 'Access' in the store, and return it's `id`.
    pub fn new(
        ownership: &Arc<RwLock<Ownership>>,
        visibility: &Arc<RwLock<Visibility>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Access>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Access {
            id,
            ownership: ownership.read().unwrap().id(),
            visibility: visibility.read().unwrap().id(),
        }));
        store.inter_access(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-ownership"}}}
    /// Navigate to [`Ownership`] across R15(1-*)
    pub fn r15_ownership<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Ownership>>> {
        vec![store.exhume_ownership(&self.ownership).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-visibility"}}}
    /// Navigate to [`Visibility`] across R14(1-*)
    pub fn r14_visibility<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Visibility>>> {
        vec![store.exhume_visibility(&self.visibility).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-backward-1_M-to-value"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R16(1-M)
    pub fn r16_x_value<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<XValue>>> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.read().unwrap().access == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
