// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_if-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::block::Block;
use crate::v2::lu_dog_vanilla::types::expression::Expression;
use crate::v2::lu_dog_vanilla::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-documentation"}}}
/// The if Expression
///
/// It does include an else, at no extra charge!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XIf {
    pub id: Uuid,
    /// R52: [`XIf`] 'false block' [`Expression`]
    pub false_block: Option<Uuid>,
    /// R44: [`XIf`] 'branches based on' [`Expression`]
    pub test: Uuid,
    /// R46: [`XIf`] 'when true, evaluates' [`Block`]
    pub true_block: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-implementation"}}}
impl XIf {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-new"}}}
    /// Inter a new 'If' in the store, and return it's `id`.
    pub fn new(
        false_block: Option<&Expression>,
        test: &Expression,
        true_block: &Block,
        store: &mut LuDogVanillaStore,
    ) -> XIf {
        let id = Uuid::new_v4();
        let new = XIf {
            id,
            false_block: false_block.as_ref().map(|expression| expression.id),
            test: test.id,
            true_block: true_block.id,
        };
        store.inter_x_if(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-cond-to-false_block"}}}
    /// Navigate to [`Expression`] across R52(1-*c)
    pub fn r52_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        match self.false_block {
            Some(ref false_block) => vec![store.exhume_expression(false_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-test"}}}
    /// Navigate to [`Expression`] across R44(1-*)
    pub fn r44_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.test).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-struct-impl-nav-forward-to-true_block"}}}
    /// Navigate to [`Block`] across R46(1-*)
    pub fn r46_block<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.true_block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_if-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::XIf(id) = expression.subtype {
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
