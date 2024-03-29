// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::negation::NEGATION;
use crate::v2::lu_dog_vec_tracy::types::not::NOT;
use crate::v2::lu_dog_vec_tracy::types::operator::Operator;
use crate::v2::lu_dog_vec_tracy::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-hybrid-documentation"}}}
/// Unary Operators
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unary {
    pub subtype: UnaryEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum UnaryEnum {
    Negation(Uuid),
    Not(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-implementation"}}}
impl Unary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-struct-impl-new_negation"}}}
    /// Inter a new Unary in the store, and return it's `id`.
    pub fn new_negation(bogus: bool, store: &mut LuDogVecTracyStore) -> Rc<RefCell<Unary>> {
        store.inter_unary(|id| {
            Rc::new(RefCell::new(Unary {
                bogus: bogus,
                subtype: UnaryEnum::Negation(NEGATION),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-struct-impl-new_not"}}}
    /// Inter a new Unary in the store, and return it's `id`.
    pub fn new_not(bogus: bool, store: &mut LuDogVecTracyStore) -> Rc<RefCell<Unary>> {
        store.inter_unary(|id| {
            Rc::new(RefCell::new(Unary {
                bogus: bogus,
                subtype: UnaryEnum::Not(NOT),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Operator>>> {
        span!("r47_operator");
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Unary(id) = operator.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-implementation"}}}
impl PartialEq for Unary {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
