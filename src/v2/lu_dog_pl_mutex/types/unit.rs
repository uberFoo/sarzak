// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::enum_field::EnumField;
use crate::v2::lu_dog_pl_mutex::types::enum_field::EnumFieldEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-struct-documentation"}}}
/// Just a marker, no other value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Unit {
    pub id: Uuid,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-implementation"}}}
impl Unit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-struct-impl-new"}}}
    /// Inter a new 'Unit' in the store, and return it's `id`.
    pub fn new(x_value: i64, store: &mut LuDogPlMutexStore) -> Arc<Mutex<Unit>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Unit { id, x_value }));
        store.inter_unit(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<EnumField>>> {
        vec![store
            .iter_enum_field()
            .find(|enum_field| {
                if let EnumFieldEnum::Unit(id) = enum_field.lock().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
