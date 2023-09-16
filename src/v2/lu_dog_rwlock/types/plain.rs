// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"plain-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::enum_field::EnumField;
use crate::v2::lu_dog_rwlock::types::enum_field::EnumFieldEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-struct-documentation"}}}
/// Just a marker, no other value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Plain {
    pub id: Uuid,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-implementation"}}}
impl Plain {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-struct-impl-new"}}}
    /// Inter a new 'Plain' in the store, and return it's `id`.
    pub fn new(x_value: i64, store: &mut LuDogRwlockStore) -> Arc<RwLock<Plain>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Plain { id, x_value }));
        store.inter_plain(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plain-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        span!("r85_enum_field");
        vec![store
            .iter_enum_field()
            .find(|enum_field| {
                if let EnumFieldEnum::Plain(id) = enum_field.read().unwrap().subtype {
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
