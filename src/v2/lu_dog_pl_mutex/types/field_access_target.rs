// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::enum_field::EnumField;
use crate::v2::lu_dog_pl_mutex::types::field::Field;
use crate::v2::lu_dog_pl_mutex::types::field_access::FieldAccess;
use crate::v2::lu_dog_pl_mutex::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FieldAccessTarget {
    pub subtype: FieldAccessTargetEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FieldAccessTargetEnum {
    EnumField(Uuid),
    Field(Uuid),
    Function(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_enum_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_enum_field(
        bogus: bool,
        subtype: &Arc<Mutex<EnumField>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<FieldAccessTarget>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(FieldAccessTarget {
            bogus: bogus,
            subtype: FieldAccessTargetEnum::EnumField(subtype.lock().id), // b
            id,
        }));
        store.inter_field_access_target(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_field(
        bogus: bool,
        subtype: &Arc<Mutex<Field>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<FieldAccessTarget>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(FieldAccessTarget {
            bogus: bogus,
            subtype: FieldAccessTargetEnum::Field(subtype.lock().id), // b
            id,
        }));
        store.inter_field_access_target(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_function"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_function(
        bogus: bool,
        subtype: &Arc<Mutex<Function>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<FieldAccessTarget>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(FieldAccessTarget {
            bogus: bogus,
            subtype: FieldAccessTargetEnum::Function(subtype.lock().id), // b
            id,
        }));
        store.inter_field_access_target(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub fn r65_field_access<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<FieldAccess>>> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.lock().field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
