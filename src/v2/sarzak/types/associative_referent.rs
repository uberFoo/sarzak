// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-use-statements"}}}
use uuid::Uuid;

use crate::v2::sarzak::types::associative::Associative;
use crate::v2::sarzak::types::cardinality::Cardinality;
use crate::v2::sarzak::types::conditionality::Conditionality;
use crate::v2::sarzak::types::object::Object;
use crate::v2::sarzak::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-documentation"}}}
/// The other objects in an Associative Relationship
///
/// This represents one of the two objects that are related in an [`Associative`] relationhip
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeReferent {
    pub description: String,
    pub id: Uuid,
    /// R88: [`AssociativeReferent`] 'has' [`Cardinality`]
    pub cardinality: Uuid,
    /// R77: [`AssociativeReferent`] 'has' [`Conditionality`]
    pub conditionality: Uuid,
    /// R25: [`AssociativeReferent`] 'has other' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-implementation"}}}
impl AssociativeReferent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-new"}}}
    /// Inter a new 'Associative Referent' in the store, and return it's `id`.
    pub fn new(
        description: String,
        cardinality: &Cardinality,
        conditionality: &Conditionality,
        obj_id: &Object,
        store: &mut SarzakStore,
    ) -> AssociativeReferent {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{:?}:{:?}:{:?}",
                description, cardinality, conditionality, obj_id
            )
            .as_bytes(),
        );
        let new = AssociativeReferent {
            description: description,
            cardinality: cardinality.id(),
            conditionality: conditionality.id(),
            obj_id: obj_id.id,
            id,
        };
        store.inter_associative_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R88(1-*)
    pub fn r88_cardinality<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Cardinality> {
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-conditionality"}}}
    /// Navigate to [`Conditionality`] across R77(1-*)
    pub fn r77_conditionality<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Conditionality> {
        vec![store.exhume_conditionality(&self.conditionality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R25(1-*)
    pub fn r25_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-cond-to-associative"}}}
    /// Navigate to [`Associative`] across R23(1-1c)
    pub fn r23c_associative<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        let associative = store
            .iter_associative()
            .find(|associative| associative.one == self.id);
        match associative {
            Some(ref associative) => vec![associative],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-cond-to-associative"}}}
    /// Navigate to [`Associative`] across R22(1-1c)
    pub fn r22c_associative<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        let associative = store
            .iter_associative()
            .find(|associative| associative.other == self.id);
        match associative {
            Some(ref associative) => vec![associative],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
