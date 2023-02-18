// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak_v2::UUID_NS;

// Referrer imports
use crate::sarzak_v2::types::associative_referent::AssociativeReferent;
use crate::sarzak_v2::types::associative_referrer::AssociativeReferrer;

use crate::sarzak_v2::store::ObjectStore as SarzakV2Store;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Associative {
    pub id: Uuid,
    pub number: i64,
    /// R23: [`Associative`] 'is formalized by' [`AssociativeReferent`]
    pub one: Uuid,
    /// R22: [`Associative`] 'is formalized by' [`AssociativeReferent`]
    pub other: Uuid,
    /// R21: [`Associative`] 'is formaized by' [`AssociativeReferrer`]
    pub from: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-implementation"}}}
impl Associative {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-new"}}}
    /// Inter a new Associative in the store, and return it's `id`.
    pub fn new(
        number: i64,
        one: &AssociativeReferent,
        other: &AssociativeReferent,
        from: &AssociativeReferrer,
        store: &mut SarzakV2Store,
    ) -> Associative {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}:{:?}", number, one, other, from).as_bytes(),
        );
        let new = Associative {
            number: number,
            one: one.id,
            other: other.id,
            from: from.id,
            id,
        };
        store.inter_associative(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-one"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-other"}}}
    /// Navigate to [`AssociativeReferent`] across R23(1-?)
    pub fn r23_associative_referent<'a>(
        &'a self,
        store: &'a SarzakV2Store,
    ) -> Vec<&AssociativeReferent> {
        vec![store.exhume_associative_referent(&self.one).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-other"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-one"}}}
    /// Navigate to [`AssociativeReferent`] across R22(1-?)
    pub fn r22_associative_referent<'a>(
        &'a self,
        store: &'a SarzakV2Store,
    ) -> Vec<&AssociativeReferent> {
        vec![store.exhume_associative_referent(&self.other).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`AssociativeReferrer`] across R21(1-?)
    pub fn r21_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakV2Store,
    ) -> Vec<&AssociativeReferrer> {
        vec![store.exhume_associative_referrer(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
