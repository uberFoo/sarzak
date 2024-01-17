// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"comparison-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::equal::EQUAL;
use crate::v2::lu_dog::types::greater_than::GREATER_THAN;
use crate::v2::lu_dog::types::greater_than_or_equal::GREATER_THAN_OR_EQUAL;
use crate::v2::lu_dog::types::less_than::LESS_THAN;
use crate::v2::lu_dog::types::less_than_or_equal::LESS_THAN_OR_EQUAL;
use crate::v2::lu_dog::types::not_equal::NOT_EQUAL;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-documentation"}}}
/// Comparison Operators
///
/// Things like == and !=, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Comparison {
    pub subtype: ComparisonEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ComparisonEnum {
    Equal(Uuid),
    GreaterThan(Uuid),
    GreaterThanOrEqual(Uuid),
    LessThan(Uuid),
    LessThanOrEqual(Uuid),
    NotEqual(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-implementation"}}}
impl Comparison {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_equal(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Comparison>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Comparison {
            bogus: bogus,
            subtype: ComparisonEnum::Equal(EQUAL),
            id,
        }));
        store.inter_comparison(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_greater_than"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_greater_than(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Comparison>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Comparison {
            bogus: bogus,
            subtype: ComparisonEnum::GreaterThan(GREATER_THAN),
            id,
        }));
        store.inter_comparison(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_greater_than_or_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_greater_than_or_equal(
        bogus: bool,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Comparison>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Comparison {
            bogus: bogus,
            subtype: ComparisonEnum::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL),
            id,
        }));
        store.inter_comparison(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_less_than"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_less_than(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Comparison>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Comparison {
            bogus: bogus,
            subtype: ComparisonEnum::LessThan(LESS_THAN),
            id,
        }));
        store.inter_comparison(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_less_than_or_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_less_than_or_equal(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Comparison>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Comparison {
            bogus: bogus,
            subtype: ComparisonEnum::LessThanOrEqual(LESS_THAN_OR_EQUAL),
            id,
        }));
        store.inter_comparison(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_not_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_not_equal(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Comparison>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Comparison {
            bogus: bogus,
            subtype: ComparisonEnum::NotEqual(NOT_EQUAL),
            id,
        }));
        store.inter_comparison(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Operator>>> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Comparison(id) = operator.borrow().subtype {
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
