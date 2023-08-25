// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"match-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec::types::pattern::Pattern;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-struct-documentation"}}}
/// Match a pattern to a scrutinee and evaluate a branch based on the results.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Match {
    pub id: usize,
    /// R87: [`Match`] 'executes a ' [`Pattern`]
    pub pattern: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-implementation"}}}
impl Match {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-struct-impl-new"}}}
    /// Inter a new 'Match' in the store, and return it's `id`.
    pub fn new(pattern: &Rc<RefCell<Pattern>>, store: &mut LuDogVecStore) -> Rc<RefCell<Match>> {
        store.inter_match(|id| {
            Rc::new(RefCell::new(Match {
                id,
                pattern: pattern.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-struct-impl-nav-forward-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-*)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Pattern>>> {
        span!("r87_pattern");
        vec![store.exhume_pattern(&self.pattern).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Match(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"match-implementation"}}}
impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
