// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referrer-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::sarzak::UUID_NS;

// Referrer imports
use crate::sarzak::types::object::Object;

// Referent imports
use crate::sarzak::types::associative::Associative;

use crate::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-documentation"}}}
/// Associative Object
///
/// This is used in an [`Associative`] relationship to point to the Associative object itself
///. It's the box with the line pointing at another line.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeReferrer {
    pub id: Uuid,
    /// R26: [`AssociativeReferrer`] 'is also an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-implementation"}}}
impl AssociativeReferrer {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-new"}}}
    /// Inter a new AssociativeReferrer in the store, and return it's `id`.
    pub fn new(obj_id: &Object, store: &mut SarzakStore) -> AssociativeReferrer {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", obj_id).as_bytes());
        let new = AssociativeReferrer {
            obj_id: obj_id.id,
            id,
        };
        store.inter_associative_referrer(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R26(1-?)
    pub fn object_r26<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-backward-one-to-associative"}}}
    /// Navigate to [`Associative`] across R21(1-1)
    pub fn associative_r21<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        vec![
            store
                .iter_associative()
                .find(|associative| associative.1.from == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
