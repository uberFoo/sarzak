// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_future-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XFuture {
    pub id: usize,
    /// R2: [`XFuture`] 'contains' [`ValueType`]
    pub x_value: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-implementation"}}}
impl XFuture {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-struct-impl-new"}}}
    /// Inter a new 'Future' in the store, and return it's `id`.
    pub fn new(
        x_value: &Rc<RefCell<ValueType>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<XFuture>> {
        store.inter_x_future(|id| {
            Rc::new(RefCell::new(XFuture {
                id,
                x_value: x_value.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-struct-impl-nav-forward-to-x_value"}}}
    /// Navigate to [`ValueType`] across R2(1-*)
    pub fn r2_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r2_value_type");
        vec![store.exhume_value_type(&self.x_value).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::XFuture(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_future-implementation"}}}
impl PartialEq for XFuture {
    fn eq(&self, other: &Self) -> bool {
        self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
