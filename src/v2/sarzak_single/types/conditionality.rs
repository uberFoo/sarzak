// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"conditionality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-use-statements"}}}
use crate::v2::sarzak_single::store::ObjectStore as SarzakSingleStore;
use crate::v2::sarzak_single::types::associative_referent::AssociativeReferent;
use crate::v2::sarzak_single::types::conditional::CONDITIONAL;
use crate::v2::sarzak_single::types::referent::Referent;
use crate::v2::sarzak_single::types::referrer::Referrer;
use crate::v2::sarzak_single::types::unconditional::UNCONDITIONAL;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Conditionality {
    Conditional(Uuid),
    Unconditional(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-implementation"}}}
impl Conditionality {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-new-impl"}}}
    /// Create a new instance of Conditionality::Conditional
    pub fn new_conditional(store: &SarzakSingleStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_conditionality(&CONDITIONAL).unwrap()
    }

    /// Create a new instance of Conditionality::Unconditional
    pub fn new_unconditional(store: &SarzakSingleStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_conditionality(&UNCONDITIONAL).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Conditional(id) => *id,
            Self::Unconditional(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R77(1-M)
    pub fn r77_associative_referent<'a>(
        &'a self,
        store: &'a SarzakSingleStore,
    ) -> Vec<Rc<RefCell<AssociativeReferent>>> {
        store
            .iter_associative_referent()
            .filter(|associative_referent| {
                associative_referent.borrow().conditionality == self.id()
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R12(1-M)
    pub fn r12_referent<'a>(&'a self, store: &'a SarzakSingleStore) -> Vec<Rc<RefCell<Referent>>> {
        store
            .iter_referent()
            .filter(|referent| referent.borrow().conditionality == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R11(1-M)
    pub fn r11_referrer<'a>(&'a self, store: &'a SarzakSingleStore) -> Vec<Rc<RefCell<Referrer>>> {
        store
            .iter_referrer()
            .filter(|referrer| referrer.borrow().conditionality == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
