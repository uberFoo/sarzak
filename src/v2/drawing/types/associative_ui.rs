// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-use-statements"}}}
use uuid::Uuid;

use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::point::Point;
use crate::v2::drawing::types::relationship_ui::RelationshipUi;
use crate::v2::sarzak::types::associative::Associative;
use serde::{Deserialize, Serialize};

use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AssociativeUi {
    pub id: Uuid,
    /// R15: [`AssociativeUi`] 'is drawn from' [`Anchor`]
    pub other: Uuid,
    /// R14: [`AssociativeUi`] 'is drawn from' [`Anchor`]
    pub one: Uuid,
    /// R16: [`AssociativeUi`] 'is drawn from' [`Anchor`]
    pub middle: Uuid,
    /// R20: [`AssociativeUi`] 'contains additional attributes to render' [`Associative`]
    pub associative_id: Uuid,
    /// R17: [`AssociativeUi`] 'is drawn from' [`Point`]
    pub from: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-implementation"}}}
impl AssociativeUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-new"}}}
    /// Inter a new 'AssociativeUI' in the store, and return it's `id`.
    pub fn new(
        other: &Anchor,
        one: &Anchor,
        middle: &Anchor,
        associative_id: &Associative,
        from: &Point,
        store: &mut DrawingStore,
    ) -> AssociativeUi {
        let id = Uuid::new_v4();
        let new = AssociativeUi {
            id: id,
            other: other.id,
            one: one.id,
            middle: middle.id,
            associative_id: associative_id.id,
            from: from.id,
        };
        store.inter_associative_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-other"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-middle"}}}
    /// Navigate to [`Anchor`] across R15(1-*)
    pub fn r15_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.other).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-one"}}}
    /// Navigate to [`Anchor`] across R14(1-*)
    pub fn r14_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.one).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-middle"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-other"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-one"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-middle"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-other"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-one"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-middle"}}}
    /// Navigate to [`Anchor`] across R16(1-*)
    pub fn r16_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.middle).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-associative_id"}}}
    /// Navigate to [`Associative`] across R20(1-*)
    pub fn r20_associative<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        vec![store.exhume_associative(&self.associative_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`Point`] across R17(1-*)
    pub fn r17_point<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Point> {
        vec![store.exhume_point(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_ui-impl-nav-subtype-to-supertype-relationship_ui"}}}
    // Navigate to [`RelationshipUi`] across R6(isa)
    pub fn r6_relationship_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&RelationshipUi> {
        vec![store.exhume_relationship_ui(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
