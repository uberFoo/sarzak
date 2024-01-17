// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"data_structure-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::enumeration::Enumeration;
use crate::v2::lu_dog::types::struct_expression::StructExpression;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DataStructure {
    pub subtype: DataStructureEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataStructureEnum {
    Enumeration(Uuid),
    WoogStruct(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-implementation"}}}
impl DataStructure {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-new_enumeration"}}}
    /// Inter a new DataStructure in the store, and return it's `id`.
    pub fn new_enumeration(
        bogus: bool,
        subtype: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<DataStructure>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(DataStructure {
            bogus: bogus,
            subtype: DataStructureEnum::Enumeration(subtype.borrow().id), // b
            id,
        }));
        store.inter_data_structure(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-new_woog_struct"}}}
    /// Inter a new DataStructure in the store, and return it's `id`.
    pub fn new_woog_struct(
        bogus: bool,
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-get-id-impl"}}}
    ) -> Rc<RefCell<DataStructure>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(DataStructure {
            bogus: bogus,
            subtype: DataStructureEnum::WoogStruct(subtype.borrow().id), // b
            id,
        }));
        store.inter_data_structure(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    /// Navigate to [`StructExpression`] across R39(1-M)
    pub fn r39_struct_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<StructExpression>>> {
        store
            .iter_struct_expression()
            .filter(|struct_expression| struct_expression.borrow().data == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
