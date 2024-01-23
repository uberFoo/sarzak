// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::enumeration::Enumeration;
use crate::v2::lu_dog_vanilla::types::value_type::ValueType;
use crate::v2::lu_dog_vanilla::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumGeneric {
    pub id: Uuid,
    pub name: String,
    /// R104: [`EnumGeneric`] 'parameterizes' [`Enumeration`]
    pub woog_enum: Uuid,
    /// R106: [`EnumGeneric`] 'next' [`EnumGeneric`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl EnumGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-new"}}}
    /// Inter a new 'Enum Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        woog_enum: &Enumeration,
        next: Option<&EnumGeneric>,
        store: &mut LuDogVanillaStore,
    ) -> EnumGeneric {
        let id = Uuid::new_v4();
        let new = EnumGeneric {
            id,
            name,
            woog_enum: woog_enum.id,
            next: next.as_ref().map(|enum_generic| enum_generic.id),
        };
        store.inter_enum_generic(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R104(1-*)
    pub fn r104_enumeration<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Enumeration> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`EnumGeneric`] across R106(1-*c)
    pub fn r106_enum_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&EnumGeneric> {
        match self.next {
            Some(ref next) => vec![store.exhume_enum_generic(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-bi-cond-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R106(1c-1c)
    pub fn r106c_enum_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&EnumGeneric> {
        let enum_generic = store
            .iter_enum_generic()
            .find(|enum_generic| enum_generic.next == Some(self.id));
        match enum_generic {
            Some(ref enum_generic) => vec![enum_generic],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R105(1-1)
    pub fn r105_enumeration<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Enumeration> {
        vec![store
            .iter_enumeration()
            .find(|enumeration| enumeration.first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::EnumGeneric(id) = value_type.subtype {
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
