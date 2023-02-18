// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"conditionality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Conditionality::Conditional(id) => *id,
            Conditionality::Unconditional(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
