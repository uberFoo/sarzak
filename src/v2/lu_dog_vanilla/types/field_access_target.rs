// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
use crate::v2::lu_dog_vanilla::types::field::Field;
use crate::v2::lu_dog_vanilla::types::field_access::FieldAccess;
use crate::v2::lu_dog_vanilla::types::function::Function;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FieldAccessTarget {
    Field(Uuid),
    Function(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-new-impl"}}}
    /// Create a new instance of FieldAccessTarget::Field
    pub fn new_field(field: &Field, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Field(field.id);
        store.inter_field_access_target(new.clone());
        new
    }

    /// Create a new instance of FieldAccessTarget::Function
    pub fn new_function(function: &Function, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Function(function.id);
        store.inter_field_access_target(new.clone());
        new
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
    pub fn r65_field_access<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FieldAccess> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.field == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
