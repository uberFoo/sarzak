// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_match-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::expression::Expression;
use crate::v2::lu_dog_vanilla::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vanilla::types::pattern::Pattern;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
        scrutinee: &Expression,
        store: &mut LuDogVanillaStore,
    ) -> XMatch {
        let id = Uuid::new_v4();
        let new = XMatch {
            id,
            uniqueness_generator,
            scrutinee: scrutinee.id,
        };
        store.inter_x_match(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-forward-to-scrutinee"}}}
    /// Navigate to [`Expression`] across R91(1-*)
    pub fn r91_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.scrutinee).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Pattern> {
        store
            .iter_pattern()
            .filter(|pattern| pattern.x_match == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::XMatch(id) = expression.subtype {
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
