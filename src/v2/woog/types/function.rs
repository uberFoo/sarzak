// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::woog::types::item::Item;
use crate::v2::woog::types::object_method::ObjectMethod;
use crate::v2::woog::types::parameter::Parameter;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-documentation"}}}
/// A function
///
/// This is a plain old function, distinct from an [`Object Method`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Function {
    pub subtype: FunctionEnum,
    pub description: String,
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FunctionEnum {
    ObjectMethod(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new_object_method"}}}
    /// Inter a new Function in the store, and return it's `id`.
    pub fn new_object_method(
        description: String,
        name: String,
        subtype: &Arc<RwLock<ObjectMethod>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Function>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Function {
            description: description,
            name: name,
            subtype: FunctionEnum::ObjectMethod(subtype.read().unwrap().id),
            id,
        }));
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R5(1c-1c)
    pub fn r5c_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r5_parameter");
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.read().unwrap().function == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R26(isa)
    pub fn r26_item<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r26_item");
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
