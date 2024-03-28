// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"comparison-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::equal::EQUAL;
use crate::v2::lu_dog_pl_vec::types::greater_than::GREATER_THAN;
use crate::v2::lu_dog_pl_vec::types::greater_than_or_equal::GREATER_THAN_OR_EQUAL;
use crate::v2::lu_dog_pl_vec::types::less_than::LESS_THAN;
use crate::v2::lu_dog_pl_vec::types::less_than_or_equal::LESS_THAN_OR_EQUAL;
use crate::v2::lu_dog_pl_vec::types::not_equal::NOT_EQUAL;
use crate::v2::lu_dog_pl_vec::types::operator::Operator;
use crate::v2::lu_dog_pl_vec::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-documentation"}}}
/// Comparison Operators
///
/// Things like == and !=, etc.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comparison {
    pub subtype: ComparisonEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_equal(bogus: bool, store: &mut LuDogPlVecStore) -> Arc<RwLock<Comparison>> {
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                bogus: bogus,
                subtype: ComparisonEnum::Equal(EQUAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_greater_than"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_greater_than(bogus: bool, store: &mut LuDogPlVecStore) -> Arc<RwLock<Comparison>> {
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                bogus: bogus,
                subtype: ComparisonEnum::GreaterThan(GREATER_THAN),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_greater_than_or_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_greater_than_or_equal(
        bogus: bool,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Comparison>> {
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                bogus: bogus,
                subtype: ComparisonEnum::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_less_than"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_less_than(bogus: bool, store: &mut LuDogPlVecStore) -> Arc<RwLock<Comparison>> {
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                bogus: bogus,
                subtype: ComparisonEnum::LessThan(LESS_THAN),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_less_than_or_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_less_than_or_equal(
        bogus: bool,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Comparison>> {
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                bogus: bogus,
                subtype: ComparisonEnum::LessThanOrEqual(LESS_THAN_OR_EQUAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-struct-impl-new_not_equal"}}}
    /// Inter a new Comparison in the store, and return it's `id`.
    pub fn new_not_equal(bogus: bool, store: &mut LuDogPlVecStore) -> Arc<RwLock<Comparison>> {
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                bogus: bogus,
                subtype: ComparisonEnum::NotEqual(NOT_EQUAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Operator>>> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Comparison(id) = operator.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"comparison-implementation"}}}
impl PartialEq for Comparison {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}