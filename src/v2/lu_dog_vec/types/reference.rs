// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-documentation"}}}
/// A Reference to a Type
///
/// This type depends on instance pointers being unique.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reference {
    pub address: Uuid,
    pub id: usize,
    pub is_valid: bool,
    /// R35: [`Reference`] '' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl Reference {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new"}}}
    /// Inter a new 'Reference' in the store, and return it's `id`.
    pub fn new(
        address: Uuid,
        is_valid: bool,
        ty: &Rc<RefCell<ValueType>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Reference>> {
        store.inter_reference(|id| {
            Rc::new(RefCell::new(Reference {
                address,
                id,
                is_valid,
                ty: ty.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R35(1-*)
    pub fn r35_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r35_value_type");
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Reference(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.is_valid == other.is_valid && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
