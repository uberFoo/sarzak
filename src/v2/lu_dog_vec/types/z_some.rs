// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_some-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec::types::woog_option::WoogOption;
use crate::v2::lu_dog_vec::types::woog_option::WoogOptionEnum;
use crate::v2::lu_dog_vec::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-documentation"}}}
/// Some Type
///
/// This type wraps another. It's used by the supertype, `[Option]`, to represent a type that
///  may or may not exist.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ZSome {
    pub id: usize,
    /// R23: [`ZSome`] 'contains' [`XValue`]
    pub inner: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-implementation"}}}
impl ZSome {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-new"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub fn new(inner: &Rc<RefCell<XValue>>, store: &mut LuDogVecStore) -> Rc<RefCell<ZSome>> {
        store.inter_z_some(|id| {
            Rc::new(RefCell::new(ZSome {
                id,
                inner: inner.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-nav-forward-to-inner"}}}
    /// Navigate to [`XValue`] across R23(1-*)
    pub fn r23_x_value<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r23_x_value");
        vec![store.exhume_x_value(&self.inner).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ZSome(id) = expression.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-woog_option"}}}
    // Navigate to [`WoogOption`] across R3(isa)
    pub fn r3_woog_option<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<WoogOption>>> {
        span!("r3_woog_option");
        vec![store
            .iter_woog_option()
            .find(|woog_option| {
                if let WoogOptionEnum::ZSome(id) = woog_option.borrow().subtype {
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
