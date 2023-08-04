// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-use-statements"}}}
use crate::v2::drawing::store::ObjectStore as DrawingStore;
use crate::v2::drawing::types::associative_ui::AssociativeUi;
use crate::v2::drawing::types::binary_ui::BinaryUi;
use crate::v2::drawing::types::isa_ui::IsaUi;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-enum-documentation"}}}
/// Additional information necessary to render relationships in the user interface.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RelationshipUi {
    AssociativeUi(Uuid),
    BinaryUi(Uuid),
    IsaUi(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-implementation"}}}
impl RelationshipUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-new-impl"}}}
    /// Create a new instance of RelationshipUi::AssociativeUi
    pub fn new_associative_ui(
        associative_ui: &Rc<RefCell<AssociativeUi>>,
        store: &mut DrawingStore,
    ) -> Rc<RefCell<Self>> {
        let id = associative_ui.borrow().id;
        if let Some(associative_ui) = store.exhume_relationship_ui(&id) {
            associative_ui
        } else {
            let new = Rc::new(RefCell::new(Self::AssociativeUi(id)));
            store.inter_relationship_ui(new.clone());
            new
        }
    }

    /// Create a new instance of RelationshipUi::BinaryUi
    pub fn new_binary_ui(
        binary_ui: &Rc<RefCell<BinaryUi>>,
        store: &mut DrawingStore,
    ) -> Rc<RefCell<Self>> {
        let id = binary_ui.borrow().id;
        if let Some(binary_ui) = store.exhume_relationship_ui(&id) {
            binary_ui
        } else {
            let new = Rc::new(RefCell::new(Self::BinaryUi(id)));
            store.inter_relationship_ui(new.clone());
            new
        }
    }

    /// Create a new instance of RelationshipUi::IsaUi
    pub fn new_isa_ui(isa_ui: &Rc<RefCell<IsaUi>>, store: &mut DrawingStore) -> Rc<RefCell<Self>> {
        let id = isa_ui.borrow().id;
        if let Some(isa_ui) = store.exhume_relationship_ui(&id) {
            isa_ui
        } else {
            let new = Rc::new(RefCell::new(Self::IsaUi(id)));
            store.inter_relationship_ui(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_ui-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::AssociativeUi(id) => *id,
            Self::BinaryUi(id) => *id,
            Self::IsaUi(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
