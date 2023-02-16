// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::drawing::UUID_NS;

// Referrer imports
use crate::drawing::types::anchor::Anchor;
use crate::drawing::types::associative::Associative;
use crate::drawing::types::point::Point;

use crate::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeUi {
    pub id: Uuid,
    /// R16: [`AssociativeUi`] 'anchors' [`Anchor`]
    pub middle: Option<Uuid>,
    /// R14: [`AssociativeUi`] 'anchors' [`Anchor`]
    pub one: Option<Uuid>,
    /// R15: [`AssociativeUi`] 'anchors' [`Anchor`]
    pub other: Option<Uuid>,
    /// R20: [`AssociativeUi`] 'is represented in the UI by' [`Associative`]
    pub associative_id: Option<Uuid>,
    /// R17: [`AssociativeUi`] 'is drawn to' [`Point`]
    pub from: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-implementation"}}}
impl AssociativeUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-new"}}}
    /// Inter a new AssociativeUi in the store, and return it's `id`.
    pub fn new(
        middle: Option<&Anchor>,
        one: Option<&Anchor>,
        other: Option<&Anchor>,
        associative_id: Option<&Associative>,
        from: Option<&Point>,
        store: &mut DrawingStore,
    ) -> AssociativeUi {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}:{:?}:{:?}:{:?}:{:?}",
                middle, one, other, associative_id, from
            )
            .as_bytes(),
        );
        let new = AssociativeUi {
            middle: middle.map(|anchor| anchor.id),
            one: one.map(|anchor| anchor.id),
            other: other.map(|anchor| anchor.id),
            associative_id: associative_id.map(|associative| associative.id),
            from: from.map(|point| point.id),
            id,
        };
        store.inter_associative_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-middle"}}}
    /// Navigate to [`Anchor`] across R16(1-?c)
    pub fn anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        match self.middle {
            Some(ref middle) => vec![store.exhume_anchor(middle).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-one"}}}
    /// Navigate to [`Anchor`] across R14(1-?c)
    pub fn anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        match self.one {
            Some(ref one) => vec![store.exhume_anchor(one).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-other"}}}
    /// Navigate to [`Anchor`] across R15(1-?c)
    pub fn anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        match self.other {
            Some(ref other) => vec![store.exhume_anchor(other).unwrap()],
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-one"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-middle"}}}
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-associative_id"}}}
    /// Navigate to [`Associative`] across R20(1-?c)
    pub fn associative<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Associative> {
        match self.associative_id {
            Some(ref associative_id) => vec![store.exhume_associative(associative_id).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-cond-to-from"}}}
    /// Navigate to [`Point`] across R17(1-?c)
    pub fn point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        match self.from {
            Some(ref from) => vec![store.exhume_point(from).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
