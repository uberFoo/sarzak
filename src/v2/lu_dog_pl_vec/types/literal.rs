// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_pl_vec::types::expression::Expression;
use crate::v2::lu_dog_pl_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_pl_vec::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_pl_vec::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_pl_vec::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Literal {
    pub subtype: LiteralEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum LiteralEnum {
    BooleanLiteral(usize),
    FloatLiteral(usize),
    IntegerLiteral(usize),
    StringLiteral(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl Literal {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_boolean_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_boolean_literal(
        bogus: bool,
        subtype: &Arc<RwLock<BooleanLiteral>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::BooleanLiteral(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_float_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_float_literal(
        bogus: bool,
        subtype: &Arc<RwLock<FloatLiteral>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::FloatLiteral(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_integer_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_integer_literal(
        bogus: bool,
        subtype: &Arc<RwLock<IntegerLiteral>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::IntegerLiteral(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_string_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_string_literal(
        bogus: bool,
        subtype: &Arc<RwLock<StringLiteral>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::StringLiteral(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Literal(id) = expression.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
