// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_struct-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::data_structure::DataStructure;
use crate::v2::lu_dog_vec::types::data_structure::DataStructureEnum;
use crate::v2::lu_dog_vec::types::field::Field;
use crate::v2::lu_dog_vec::types::field_access::FieldAccess;
use crate::v2::lu_dog_vec::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_vec::types::item::Item;
use crate::v2::lu_dog_vec::types::item::ItemEnum;
use crate::v2::lu_dog_vec::types::struct_generic::StructGeneric;
use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-documentation"}}}
/// A Type from the Model
///
/// This is really just an alias for `[Object]`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WoogStruct {
    pub id: usize,
    pub name: String,
    pub x_path: String,
    /// R102: [`WoogStruct`] 'may have a ' [`StructGeneric`]
    pub first_generic: Option<usize>,
    /// R4: [`WoogStruct`] 'mirrors an' [`Object`]
    pub object: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-implementation"}}}
impl WoogStruct {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-new"}}}
    /// Inter a new 'Struct' in the store, and return it's `id`.
    pub fn new(
        name: String,
        x_path: String,
        first_generic: Option<&Rc<RefCell<StructGeneric>>>,
        object: Option<&Object>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<WoogStruct>> {
        store.inter_woog_struct(|id| {
            Rc::new(RefCell::new(WoogStruct {
                id,
                name: name.to_owned(),
                x_path: x_path.to_owned(),
                first_generic: first_generic.map(|struct_generic| struct_generic.borrow().id),
                object: object.as_ref().map(|object| object.id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-forward-cond-to-first_generic"}}}
    /// Navigate to [`StructGeneric`] across R102(1-*c)
    pub fn r102_struct_generic<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<StructGeneric>>> {
        match self.first_generic {
            Some(ref first_generic) => vec![store.exhume_struct_generic(&first_generic).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-forward-cond-to-object"}}}
    /// Navigate to [`Object`] across R4(1-*c)
    pub fn r4_object<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<Object>>> {
        match self.object {
            Some(ref object) => vec![store.exhume_object(&object).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R7(1-M)
    pub fn r7_field<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Field>>> {
        store
            .iter_field()
            .filter(|field| field.borrow().x_model == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R66(1-M)
    pub fn r66_field_access<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FieldAccess>>> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.borrow().woog_struct == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-cond-to-implementation"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-cond-to-implementation_block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-one-bi-cond-to-implementation_block"}}}
    /// Navigate to [`ImplementationBlock`] across R8(1c-1c)
    pub fn r8c_implementation_block<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ImplementationBlock>>> {
        let implementation_block = store
            .iter_implementation_block()
            .find(|implementation_block| implementation_block.borrow().model_type == Some(self.id));
        match implementation_block {
            Some(ref implementation_block) => vec![implementation_block.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-struct_generic"}}}
    /// Navigate to [`StructGeneric`] across R100(1-M)
    pub fn r100_struct_generic<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<StructGeneric>>> {
        store
            .iter_struct_generic()
            .filter(|struct_generic| struct_generic.borrow().woog_struct == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-data_structure"}}}
    // Navigate to [`DataStructure`] across R95(isa)
    pub fn r95_data_structure<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<DataStructure>>> {
        vec![store
            .iter_data_structure()
            .find(|data_structure| {
                if let DataStructureEnum::WoogStruct(id) = data_structure.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Item>>> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::WoogStruct(id) = item.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::WoogStruct(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-implementation"}}}
impl PartialEq for WoogStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.x_path == other.x_path
            && self.first_generic == other.first_generic
            && self.object == other.object
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
