// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::data_structure::DataStructure;
use crate::v2::lu_dog_vanilla::types::data_structure::DataStructureEnum;
use crate::v2::lu_dog_vanilla::types::enum_field::EnumField;
use crate::v2::lu_dog_vanilla::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog_vanilla::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vanilla::types::item::Item;
use crate::v2::lu_dog_vanilla::types::item::ItemEnum;
use crate::v2::lu_dog_vanilla::types::value_type::ValueType;
use crate::v2::lu_dog_vanilla::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-documentation"}}}
/// An Enumeration
///
/// An enumeration is an algebraic type that takes on one if it’s fielsd, another type. as
///  it’s value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Enumeration {
    pub id: Uuid,
    pub name: String,
    pub x_path: String,
    /// R105: [`Enumeration`] 'may have a first' [`EnumGeneric`]
    pub first_generic: Option<Uuid>,
    /// R84: [`Enumeration`] 'may have an' [`ImplementationBlock`]
    pub implementation: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-implementation"}}}
impl Enumeration {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-new"}}}
    /// Inter a new 'Enumeration' in the store, and return it's `id`.
    pub fn new(
        name: String,
        x_path: String,
        first_generic: Option<&EnumGeneric>,
        implementation: Option<&ImplementationBlock>,
        store: &mut LuDogVanillaStore,
    ) -> Enumeration {
        let id = Uuid::new_v4();
        let new = Enumeration {
            id,
            name,
            x_path,
            first_generic: first_generic.as_ref().map(|enum_generic| enum_generic.id),
            implementation: implementation
                .as_ref()
                .map(|implementation_block| implementation_block.id),
        };
        store.inter_enumeration(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-forward-cond-to-first_generic"}}}
    /// Navigate to [`EnumGeneric`] across R105(1-*c)
    pub fn r105_enum_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&EnumGeneric> {
        match self.first_generic {
            Some(ref first_generic) => vec![store.exhume_enum_generic(first_generic).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-forward-cond-to-implementation"}}}
    /// Navigate to [`ImplementationBlock`] across R84(1-*c)
    pub fn r84_implementation_block<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&ImplementationBlock> {
        match self.implementation {
            Some(ref implementation) => {
                vec![store.exhume_implementation_block(implementation).unwrap()]
            }
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-1_M-to-enum_field"}}}
    /// Navigate to [`EnumField`] across R88(1-M)
    pub fn r88_enum_field<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&EnumField> {
        store
            .iter_enum_field()
            .filter(|enum_field| enum_field.woog_enum == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-1_M-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R104(1-M)
    pub fn r104_enum_generic<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&EnumGeneric> {
        store
            .iter_enum_generic()
            .filter(|enum_generic| enum_generic.woog_enum == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-data_structure"}}}
    // Navigate to [`DataStructure`] across R95(isa)
    pub fn r95_data_structure<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&DataStructure> {
        vec![store
            .iter_data_structure()
            .find(|data_structure| {
                if let DataStructureEnum::Enumeration(id) = data_structure.subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Item> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Enumeration(id) = item.subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Enumeration(id) = value_type.subtype {
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
