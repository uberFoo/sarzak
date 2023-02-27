// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog_2::UUID_NS;

// Referrer imports
use crate::v2::woog_2::types::ownership::Ownership;
use crate::v2::woog_2::types::visibility::Visibility;

// Referent imports
use crate::v2::woog_2::types::value::Value;

use crate::v2::woog_2::store::ObjectStore as Woog2Store;
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
    /// Inter a new Access in the store, and return it's `id`.
    pub fn new(ownership: &Ownership, visibility: &Visibility, store: &mut Woog2Store) -> Access {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}", ownership, visibility).as_bytes(),
        );
        let new = Access {
            ownership: ownership.id(),
            visibility: visibility.id(),
            id,
        };
        store.inter_access(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-mutability"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-ownership"}}}
    /// Navigate to [`Ownership`] across R15(1-*)
    pub fn r15_ownership<'a>(&'a self, store: &'a Woog2Store) -> Vec<&Ownership> {
        vec![store.exhume_ownership(&self.ownership).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-forward-to-visibility"}}}
    /// Navigate to [`Visibility`] across R14(1-*)
    pub fn r14_visibility<'a>(&'a self, store: &'a Woog2Store) -> Vec<&Visibility> {
        vec![store.exhume_visibility(&self.visibility).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"access-struct-impl-nav-backward-1_M-to-value"}}}
    /// Navigate to [`Value`] across R16(1-M)
    pub fn r16_value<'a>(&'a self, store: &'a Woog2Store) -> Vec<&Value> {
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
