// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_match-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::pattern::Pattern;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-documentation"}}}
/// Match a pattern to a scrutinee and evaluate a branch based on the results.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XMatch {
    pub id: Uuid,
    pub uniqueness_generator: Uuid,
    /// R91: [`XMatch`] 'deconstructs ' [`Expression`]
    pub scrutinee: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-implementation"}}}
impl XMatch {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-new"}}}
    /// Inter a new 'Match' in the store, and return it's `id`.
    pub fn new(
        uniqueness_generator: Uuid,
        scrutinee: &Rc<RefCell<Expression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<XMatch>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(XMatch {
            id,
            uniqueness_generator,
            scrutinee: scrutinee.borrow().id(),
        }));
        store.inter_x_match(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-forward-to-scrutinee"}}}
    /// Navigate to [`Expression`] across R91(1-*)
    pub fn r91_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r91_expression");
        vec![store.exhume_expression(&self.scrutinee).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Pattern>>> {
        span!("r87_pattern");
        store
            .iter_pattern()
            .filter(|pattern| pattern.borrow().x_match == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}