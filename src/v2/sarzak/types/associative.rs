// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::sarzak::types::an_associative_referent::AnAssociativeReferent;
use crate::v2::sarzak::types::associative_referrer::AssociativeReferrer;
use crate::v2::sarzak::types::relationship::Relationship;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Associative {
    pub id: Uuid,
    pub number: i64,
    /// R21: [`Associative`] 'is formalized by' [`AssociativeReferrer`]
    pub from: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-implementation"}}}
impl Associative {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-new"}}}
    /// Inter a new 'Associative' in the store, and return it's `id`.
    pub fn new(
        number: i64,
        from: &Arc<RwLock<AssociativeReferrer>>,
        store: &mut SarzakStore,
    ) -> Arc<RwLock<Associative>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Associative {
            id,
            number,
            from: from.read().unwrap().id,
        }));
        store.inter_associative(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`AssociativeReferrer`] across R21(1-*)
    pub fn r21_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<Arc<RwLock<AssociativeReferrer>>> {
        vec![store.exhume_associative_referrer(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-backward-assoc-many-to-an_associative_referent"}}}
    /// Navigate to [`AnAssociativeReferent`] across R22(1-M)
    pub fn r22_an_associative_referent<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<Arc<RwLock<AnAssociativeReferent>>> {
        store
            .iter_an_associative_referent()
            .filter(|an_associative_referent| {
                an_associative_referent.read().unwrap().associative == self.id
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-impl-nav-subtype-to-supertype-relationship"}}}
    // Navigate to [`Relationship`] across R4(isa)
    pub fn r4_relationship<'a>(&'a self, store: &'a SarzakStore) -> Vec<Arc<RwLock<Relationship>>> {
        vec![store.exhume_relationship(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
