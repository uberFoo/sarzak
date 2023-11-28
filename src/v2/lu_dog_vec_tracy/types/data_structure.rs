// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"data_structure-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::enumeration::Enumeration;
use crate::v2::lu_dog_vec_tracy::types::struct_expression::StructExpression;
use crate::v2::lu_dog_vec_tracy::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataStructure {
    pub subtype: DataStructureEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum DataStructureEnum {
    Enumeration(usize),
    WoogStruct(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-implementation"}}}
impl DataStructure {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-new_enumeration"}}}
    /// Inter a new DataStructure in the store, and return it's `id`.
    pub fn new_enumeration(
        subtype: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<DataStructure>> {
        store.inter_data_structure(|id| {
            Rc::new(RefCell::new(DataStructure {
                subtype: DataStructureEnum::Enumeration(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-new_woog_struct"}}}
    /// Inter a new DataStructure in the store, and return it's `id`.
    pub fn new_woog_struct(
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<DataStructure>> {
        store.inter_data_structure(|id| {
            Rc::new(RefCell::new(DataStructure {
                subtype: DataStructureEnum::WoogStruct(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    /// Navigate to [`StructExpression`] across R39(1-M)
    pub fn r39_struct_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<StructExpression>>> {
        span!("r39_struct_expression");
        store
            .iter_struct_expression()
            .filter(|struct_expression| struct_expression.borrow().data == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-implementation"}}}
impl PartialEq for DataStructure {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
