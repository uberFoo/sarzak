// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"conditionality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-use-statements"}}}
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::sarzak::types::associative_referent::AssociativeReferent;
use crate::v2::sarzak::types::conditional::CONDITIONAL;
use crate::v2::sarzak::types::referent::Referent;
use crate::v2::sarzak::types::referrer::Referrer;
use crate::v2::sarzak::types::unconditional::UNCONDITIONAL;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Conditionality {
    Conditional(Uuid),
    Unconditional(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-implementation"}}}
impl Conditionality {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-new-impl"}}}
    /// Create a new instance of Conditionality::Conditional
    pub fn new_conditional() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Conditional(CONDITIONAL)
    }

    /// Create a new instance of Conditionality::Unconditional
    pub fn new_unconditional() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Unconditional(UNCONDITIONAL)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Conditionality::Conditional(id) => *id,
            Conditionality::Unconditional(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R77(1-M)
    pub fn r77_associative_referent<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<&AssociativeReferent> {
        store
            .iter_associative_referent()
            .filter(|associative_referent| associative_referent.conditionality == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R12(1-M)
    pub fn r12_referent<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Referent> {
        store
            .iter_referent()
            .filter(|referent| referent.conditionality == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R11(1-M)
    pub fn r11_referrer<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Referrer> {
        store
            .iter_referrer()
            .filter(|referrer| referrer.conditionality == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
