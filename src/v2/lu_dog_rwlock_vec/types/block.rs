// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::body::Body;
use crate::v2::lu_dog_rwlock_vec::types::body::BodyEnum;
use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock_vec::types::for_loop::ForLoop;
use crate::v2::lu_dog_rwlock_vec::types::lambda::Lambda;
use crate::v2::lu_dog_rwlock_vec::types::statement::Statement;
use crate::v2::lu_dog_rwlock_vec::types::x_if::XIf;
use crate::v2::lu_dog_rwlock_vec::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-documentation"}}}
/// A Block Expression
///
/// A block expression is an expression that consists of an ordered set of statements, living
///  between an opening `{`, and a closing `}`.
///
///  Given that it's an expression it has a Type. The type is the value of the last expression
///  in the block, if it's not closed by a `;`. If the last statement is termintat thusly, then
///  the value is `[Empty]`, or `()`.
///
/// The `bug` attribute is there to force the compiler to generate code. Apparently there's
///  some bug in grace that's causing this to be generated as a const. I don't want to get into
///  it, and this is the most expedient solution.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub bug: Uuid,
    pub id: usize,
    /// R71: [`Block`] '' [`Statement`]
    pub statement: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new(
        bug: Uuid,
        statement: Option<&Arc<RwLock<Statement>>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Block>> {
        store.inter_block(|id| {
            Arc::new(RwLock::new(Block {
                bug,
                id,
                statement: statement.map(|statement| statement.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R71(1-*c)
    pub fn r71_statement<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r71_statement");
        match self.statement {
            Some(ref statement) => vec![store.exhume_statement(&statement).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub fn r43_for_loop<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<ForLoop>>> {
        span!("r43_for_loop");
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.read().unwrap().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-function"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R46(1-M)
    pub fn r46_x_if<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<XIf>>> {
        span!("r46_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.read().unwrap().true_block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub fn r52_x_if<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<XIf>>> {
        span!("r52_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.read().unwrap().false_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-lambda"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-lambda"}}}
    /// Navigate to [`Lambda`] across R73(1c-1c)
    pub fn r73c_lambda<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Lambda>>> {
        span!("r73_lambda");
        let lambda = store
            .iter_lambda()
            .find(|lambda| lambda.read().unwrap().block == Some(self.id));
        match lambda {
            Some(ref lambda) => vec![lambda.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub fn r18_statement<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r18_statement");
        store
            .iter_statement()
            .filter(|statement| statement.read().unwrap().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R33(1-M)
    pub fn r33_x_value<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r33_x_value");
        store
            .iter_x_value()
            .filter(|x_value| x_value.read().unwrap().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub fn r80_body<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Body>>> {
        span!("r80_body");
        vec![store
            .iter_body()
            .find(|body| {
                if let BodyEnum::Block(id) = body.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Block(id) = expression.read().unwrap().subtype {
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
