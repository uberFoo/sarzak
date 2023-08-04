// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::relationship_ui::RelationshipUi;
use crate::v2::sarzak::types::binary::Binary;
use serde::{Deserialize, Serialize};

use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-documentation"}}}
/// This represents additional information necessary to render a `Binary` relationship in the
///  user interface.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BinaryUi {
    pub id: Uuid,
    /// R12: [`BinaryUi`] 'contains additional attributes to render' [`Binary`]
    pub binary_id: Uuid,
    /// R7: [`BinaryUi`] 'is drawn from' [`Anchor`]
    pub from: Uuid,
    /// R8: [`BinaryUi`] 'is drawn to' [`Anchor`]
    pub to: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-implementation"}}}
impl BinaryUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-new"}}}
    /// Inter a new 'BinaryUI' in the store, and return it's `id`.
    pub fn new(
        binary_id: &Binary,
        from: &Rc<RefCell<Anchor>>,
        to: &Rc<RefCell<Anchor>>,
        store: &mut DrawingStore,
    ) -> Rc<RefCell<BinaryUi>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(BinaryUi {
            id,
            binary_id: binary_id.id,
            from: from.borrow().id,
            to: to.borrow().id,
        }));
        store.inter_binary_ui(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-from"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-to"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-binary_id"}}}
    /// Navigate to [`Binary`] across R12(1-*)
    pub fn r12_binary<'a>(&'a self, store: &'a SarzakStore) -> Vec<Rc<RefCell<Binary>>> {
        vec![store.exhume_binary(&self.binary_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`Anchor`] across R7(1-*)
    pub fn r7_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<Anchor>>> {
        span!("r7_anchor");
        vec![store.exhume_anchor(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-binary_id"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-struct-impl-nav-forward-to-to"}}}
    /// Navigate to [`Anchor`] across R8(1-*)
    pub fn r8_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<Anchor>>> {
        span!("r8_anchor");
        vec![store.exhume_anchor(&self.to).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary_ui-impl-nav-subtype-to-supertype-relationship_ui"}}}
    // Navigate to [`RelationshipUi`] across R6(isa)
    pub fn r6_relationship_ui<'a>(
        &'a self,
        store: &'a DrawingStore,
    ) -> Vec<Rc<RefCell<RelationshipUi>>> {
        span!("r6_relationship_ui");
        vec![store.exhume_relationship_ui(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
