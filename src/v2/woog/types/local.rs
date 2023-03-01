// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"local-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog::UUID_NS;

use crate::v2::woog::types::variable::Variable;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local-struct-documentation"}}}
/// A Local Variable
///
/// A plain old variable. It's got a name.
///
/// At some point I'm going to have to start thinking about scopes. 🤔
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Local {
    pub id: Uuid,
    pub name: String,
    pub value: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local-implementation"}}}
impl Local {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local-struct-impl-new"}}}
    /// Inter a new 'Local' in the store, and return it's `id`.
    pub fn new(name: String, value: String, store: &mut WoogStore) -> Local {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{}", name, value).as_bytes());
        let new = Local {
            name: name,
            value: value,
            id,
        };
        store.inter_local(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"local-impl-nav-subtype-to-supertype-variable"}}}
    // Navigate to [`Variable`] across R8(isa)
    pub fn r8_variable<'a>(&'a self, store: &'a WoogStore) -> Vec<&Variable> {
        vec![store.exhume_variable(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}