// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::data_structure::DataStructure;
use crate::v2::lu_dog_ndrwlock_vec::types::data_structure::DataStructureEnum;
use crate::v2::lu_dog_ndrwlock_vec::types::enum_field::EnumField;
use crate::v2::lu_dog_ndrwlock_vec::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog_ndrwlock_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_ndrwlock_vec::types::item::Item;
use crate::v2::lu_dog_ndrwlock_vec::types::item::ItemEnum;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-documentation"}}}
/// An Enumeration
///
/// An enumeration is an algebraic type that takes on one if it’s fielsd, another type. as
///  it’s value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enumeration {
    pub id: usize,
    pub name: String,
    pub x_path: String,
    /// R105: [`Enumeration`] 'may have a first' [`EnumGeneric`]
    pub first_generic: Option<usize>,
    /// R84: [`Enumeration`] 'may have an' [`ImplementationBlock`]
    pub implementation: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-implementation"}}}
impl Enumeration {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-new"}}}
    /// Inter a new 'Enumeration' in the store, and return it's `id`.
    pub fn new(
        name: String,
        x_path: String,
        first_generic: Option<&Arc<RwLock<EnumGeneric>>>,
        implementation: Option<&Arc<RwLock<ImplementationBlock>>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<Enumeration>> {
        store.inter_enumeration(|id| {
            Arc::new(RwLock::new(Enumeration {
                id,
                name: name.to_owned(),
                x_path: x_path.to_owned(),
                first_generic: first_generic.map(|enum_generic| enum_generic.read().unwrap().id),
                implementation: implementation
                    .map(|implementation_block| implementation_block.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-forward-cond-to-first_generic"}}}
    /// Navigate to [`EnumGeneric`] across R105(1-*c)
    pub fn r105_enum_generic<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumGeneric>>> {
        match self.first_generic {
            Some(ref first_generic) => vec![store.exhume_enum_generic(&first_generic).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-forward-cond-to-implementation"}}}
    /// Navigate to [`ImplementationBlock`] across R84(1-*c)
    pub fn r84_implementation_block<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ImplementationBlock>>> {
        match self.implementation {
            Some(ref implementation) => {
                vec![store.exhume_implementation_block(&implementation).unwrap()]
            }
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-one-to-enum_field"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-1_M-to-enum_field"}}}
    /// Navigate to [`EnumField`] across R88(1-M)
    pub fn r88_enum_field<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        store
            .iter_enum_field()
            .filter(|enum_field| enum_field.read().unwrap().woog_enum == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-1_M-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R104(1-M)
    pub fn r104_enum_generic<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumGeneric>>> {
        store
            .iter_enum_generic()
            .filter(|enum_generic| enum_generic.read().unwrap().woog_enum == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-data_structure"}}}
    // Navigate to [`DataStructure`] across R95(isa)
    pub fn r95_data_structure<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<DataStructure>>> {
        vec![store
            .iter_data_structure()
            .find(|data_structure| {
                if let DataStructureEnum::Enumeration(id) = data_structure.read().unwrap().subtype {
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
    pub fn r6_item<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Item>>> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Enumeration(id) = item.read().unwrap().subtype {
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
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Enumeration(id) = value_type.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-implementation"}}}
impl PartialEq for Enumeration {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.x_path == other.x_path
            && self.first_generic == other.first_generic
            && self.implementation == other.implementation
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
