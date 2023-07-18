// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock_vec::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_rwlock_vec::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_rwlock_vec::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Literal {
    pub subtype: LiteralEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
        subtype: &Arc<RwLock<BooleanLiteral>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                subtype: LiteralEnum::BooleanLiteral(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_float_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_float_literal(
        subtype: &Arc<RwLock<FloatLiteral>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                subtype: LiteralEnum::FloatLiteral(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_integer_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_integer_literal(
        subtype: &Arc<RwLock<IntegerLiteral>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                subtype: LiteralEnum::IntegerLiteral(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_string_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_string_literal(
        subtype: &Arc<RwLock<StringLiteral>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Literal>> {
        store.inter_literal(|id| {
            Arc::new(RwLock::new(Literal {
                subtype: LiteralEnum::StringLiteral(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Literal(id) = expression.read().unwrap().subtype {
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