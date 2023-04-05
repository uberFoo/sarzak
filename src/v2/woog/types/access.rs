// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::ownership::Ownership;
use crate::v2::woog::types::value::Value;
use crate::v2::woog::types::visibility::Visibility;
use crate::v2::woog::UUID_NS;
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
    pub fn new(ownership: &Ownership, visibility: &Visibility, store: &mut WoogStore) -> Access {
        let id = Uuid::new_v4();
        let new = Access {
            id: id,
            ownership: ownership.id(),
            visibility: visibility.id(),
        };
        store.inter_access(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-ownership"}}}
    /// Navigate to [`Ownership`] across R15(1-*)
    pub fn r15_ownership<'a>(&'a self, store: &'a WoogStore) -> Vec<&Ownership> {
        vec![store.exhume_ownership(&self.ownership).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-visibility"}}}
    /// Navigate to [`Visibility`] across R14(1-*)
    pub fn r14_visibility<'a>(&'a self, store: &'a WoogStore) -> Vec<&Visibility> {
        vec![store.exhume_visibility(&self.visibility).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-backward-1_M-to-value"}}}
    /// Navigate to [`Value`] across R16(1-M)
    pub fn r16_value<'a>(&'a self, store: &'a WoogStore) -> Vec<&Value> {
        store
            .iter_value()
            .filter_map(|value| {
                if value.access == self.id {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
