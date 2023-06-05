// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
use crate::v2::lu_dog_pl_mutex::types::field::Field;
use crate::v2::lu_dog_pl_mutex::types::field_access::FieldAccess;
use crate::v2::lu_dog_pl_mutex::types::function::Function;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FieldAccessTarget {
    Field(Uuid),
    Function(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-new-impl"}}}
    /// Create a new instance of FieldAccessTarget::Field
    pub fn new_field(field: &Arc<Mutex<Field>>, store: &mut LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        let id = field.lock().id;
        if let Some(field) = store.exhume_field_access_target(&id) {
            field
        } else {
            let new = Arc::new(Mutex::new(Self::Field(id)));
            store.inter_field_access_target(new.clone());
            new
        }
    }

    /// Create a new instance of FieldAccessTarget::Function
    pub fn new_function(
        function: &Arc<Mutex<Function>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = function.lock().id;
        if let Some(function) = store.exhume_field_access_target(&id) {
            function
        } else {
            let new = Arc::new(Mutex::new(Self::Function(id)));
            store.inter_field_access_target(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            FieldAccessTarget::Field(id) => *id,
            FieldAccessTarget::Function(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub fn r65_field_access<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<FieldAccess>>> {
        span!("r65_field_access");
        store
            .iter_field_access()
            .filter(|field_access| field_access.lock().field == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
