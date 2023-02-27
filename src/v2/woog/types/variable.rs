// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::v2::woog_2::types::local::Local;
use crate::v2::woog_2::types::parameter::Parameter;

use crate::v2::woog_2::store::ObjectStore as Woog2Store;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-enum-documentation"}}}
/// A Variable
///
/// Basically a name given to some memory.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Variable {
    Local(Uuid),
    Parameter(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl Variable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-new-impl"}}}
    /// Create a new instance of Variable::Local
    pub fn new_local(local: &Local, store: &mut Woog2Store) -> Self {
        let new = Self::Local(local.id);
        store.inter_variable(new.clone());
        new
    }

    /// Create a new instance of Variable::Parameter
    pub fn new_parameter(parameter: &Parameter, store: &mut Woog2Store) -> Self {
        let new = Self::Parameter(parameter.id);
        store.inter_variable(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Variable::Local(id) => *id,
            Variable::Parameter(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
