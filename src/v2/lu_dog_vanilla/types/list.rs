// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::value_type::ValueType;
use crate::v2::lu_dog_vanilla::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-documentation"}}}
/// A List
///
/// This is like an array, I guess. It's also like a `Vec<T>`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct List {
    pub id: Uuid,
    /// R36: [`List`] '' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-implementation"}}}
impl List {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-impl-new"}}}
    /// Inter a new 'List' in the store, and return it's `id`.
    pub fn new(ty: &ValueType, store: &mut LuDogVanillaStore) -> List {
        let id = Uuid::new_v4();
        let new = List { id, ty: ty.id };
        store.inter_list(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R36(1-*)
    pub fn r36_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::List(id) = value_type.subtype {
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
