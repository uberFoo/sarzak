// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::data_structure::DataStructure;
use crate::v2::lu_dog_vec_tracy::types::data_structure::DataStructureEnum;
use crate::v2::lu_dog_vec_tracy::types::enum_field::EnumField;
use crate::v2::lu_dog_vec_tracy::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vec_tracy::types::item::Item;
use crate::v2::lu_dog_vec_tracy::types::item::ItemEnum;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueType;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
        implementation: Option<&Rc<RefCell<ImplementationBlock>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Enumeration>> {
        store.inter_enumeration(|id| {
            Rc::new(RefCell::new(Enumeration {
                id,
                name: name.to_owned(),
                implementation: implementation
                    .map(|implementation_block| implementation_block.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-forward-cond-to-implementation"}}}
    /// Navigate to [`ImplementationBlock`] across R84(1-*c)
    pub fn r84_implementation_block<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ImplementationBlock>>> {
        span!("r84_implementation_block");
        match self.implementation {
            Some(ref implementation) => {
                vec![store.exhume_implementation_block(&implementation).unwrap()]
            }
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-1_M-to-enum_field"}}}
    /// Navigate to [`EnumField`] across R88(1-M)
    pub fn r88_enum_field<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<EnumField>>> {
        span!("r88_enum_field");
        store
            .iter_enum_field()
            .filter(|enum_field| enum_field.borrow().woog_enum == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-data_structure"}}}
    // Navigate to [`DataStructure`] across R95(isa)
    pub fn r95_data_structure<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<DataStructure>>> {
        span!("r95_data_structure");
        vec![store
            .iter_data_structure()
            .find(|data_structure| {
                if let DataStructureEnum::Enumeration(id) = data_structure.borrow().subtype {
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
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Enumeration(id) = item.borrow().subtype {
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Enumeration(id) = value_type.borrow().subtype {
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
        self.name == other.name && self.implementation == other.implementation
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}