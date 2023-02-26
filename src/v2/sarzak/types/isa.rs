// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::sarzak::UUID_NS;

// Referrer imports
use crate::v2::sarzak::types::supertype::Supertype;

// Referent imports
use crate::v2::sarzak::types::subtype::Subtype;

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Isa {
    pub id: Uuid,
    pub number: i64,
    /// R13: [`Isa`] 'has one' [`Supertype`]
    pub supertype: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-implementation"}}}
impl Isa {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-new"}}}
    /// Inter a new Isa in the store, and return it's `id`.
    pub fn new(number: i64, supertype: &Supertype, store: &mut SarzakStore) -> Isa {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", number, supertype).as_bytes());
        let new = Isa {
            number: number,
            supertype: supertype.id,
            id,
        };
        store.inter_isa(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-forward-to-supertype"}}}
    /// Navigate to [`Supertype`] across R13(1-*)
    pub fn r13_supertype<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Supertype> {
        vec![store.exhume_supertype(&self.supertype).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-backward-1_M-to-subtype"}}}
    /// Navigate to [`Subtype`] across R27(1-M)
    pub fn r27_subtype<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Subtype> {
        store
            .iter_subtype()
            .filter_map(|subtype| {
                if subtype.isa == self.id {
                    Some(subtype)
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
