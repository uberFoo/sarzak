// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"data_structure-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_rwlock_vec::types::struct_expression::StructExpression;
use crate::v2::lu_dog_rwlock_vec::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataStructure {
    pub subtype: DataStructureEnum,
    pub bogus: bool,
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
        bogus: bool,
        subtype: &Arc<RwLock<Enumeration>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<DataStructure>> {
        store.inter_data_structure(|id| {
            Arc::new(RwLock::new(DataStructure {
                bogus: bogus,
                subtype: DataStructureEnum::Enumeration(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-new_woog_struct"}}}
    /// Inter a new DataStructure in the store, and return it's `id`.
    pub fn new_woog_struct(
        bogus: bool,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<DataStructure>> {
        store.inter_data_structure(|id| {
            Arc::new(RwLock::new(DataStructure {
                bogus: bogus,
                subtype: DataStructureEnum::WoogStruct(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    /// Navigate to [`StructExpression`] across R39(1-M)
    pub fn r39_struct_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<StructExpression>>> {
        store
            .iter_struct_expression()
            .filter(|struct_expression| struct_expression.read().unwrap().data == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-implementation"}}}
impl PartialEq for DataStructure {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
