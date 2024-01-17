// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::and::AND;
use crate::v2::lu_dog_vec_tracy::types::binary::Binary;
use crate::v2::lu_dog_vec_tracy::types::binary::BinaryEnum;
use crate::v2::lu_dog_vec_tracy::types::or::OR;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-documentation"}}}
/// A Boolean Operaator
///
/// There are two — || and &&.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BooleanOperator {
    pub subtype: BooleanOperatorEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BooleanOperatorEnum {
    And(Uuid),
    Or(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-implementation"}}}
impl BooleanOperator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-struct-impl-new_and"}}}
    /// Inter a new BooleanOperator in the store, and return it's `id`.
    pub fn new_and(bogus: bool, store: &mut LuDogVecTracyStore) -> Rc<RefCell<BooleanOperator>> {
        store.inter_boolean_operator(|id| {
            Rc::new(RefCell::new(BooleanOperator {
                bogus: bogus,
                subtype: BooleanOperatorEnum::And(AND),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-struct-impl-new_or"}}}
    /// Inter a new BooleanOperator in the store, and return it's `id`.
    pub fn new_or(bogus: bool, store: &mut LuDogVecTracyStore) -> Rc<RefCell<BooleanOperator>> {
        store.inter_boolean_operator(|id| {
            Rc::new(RefCell::new(BooleanOperator {
                bogus: bogus,
                subtype: BooleanOperatorEnum::Or(OR),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-impl-nav-subtype-to-supertype-binary"}}}
    // Navigate to [`Binary`] across R48(isa)
    pub fn r48_binary<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Binary>>> {
        span!("r48_binary");
        vec![store
            .iter_binary()
            .find(|binary| {
                if let BinaryEnum::BooleanOperator(id) = binary.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-implementation"}}}
impl PartialEq for BooleanOperator {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
