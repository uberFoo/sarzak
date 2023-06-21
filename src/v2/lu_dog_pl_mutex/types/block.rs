// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::expression::Expression;
use crate::v2::lu_dog_pl_mutex::types::for_loop::ForLoop;
use crate::v2::lu_dog_pl_mutex::types::function::Function;
use crate::v2::lu_dog_pl_mutex::types::statement::Statement;
use crate::v2::lu_dog_pl_mutex::types::x_if::XIf;
use crate::v2::lu_dog_pl_mutex::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
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
    pub id: Uuid,
    /// R71: [`Block`] '' [`Statement`]
    pub statement: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new(
        bug: Uuid,
        statement: Option<&Arc<Mutex<Statement>>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Block>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Block {
            bug,
            id,
            statement: statement.map(|statement| statement.lock().id),
        }));
        store.inter_block(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R71(1-*c)
    pub fn r71_statement<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Statement>>> {
        span!("r71_statement");
        match self.statement {
            Some(ref statement) => vec![store.exhume_statement(statement).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub fn r43_for_loop<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<ForLoop>>> {
        span!("r43_for_loop");
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.lock().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-function"}}}
    /// Navigate to [`Function`] across R19(1-1c)
    pub fn r19c_function<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Function>>> {
        span!("r19_function");
        let function = store
            .iter_function()
            .find(|function| function.lock().block == self.id);
        match function {
            Some(ref function) => vec![function.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub fn r52_x_if<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XIf>>> {
        span!("r52_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.lock().false_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R46(1-M)
    pub fn r46_x_if<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XIf>>> {
        span!("r46_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.lock().true_block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub fn r18_statement<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Statement>>> {
        span!("r18_statement");
        store
            .iter_statement()
            .filter(|statement| statement.lock().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R33(1-M)
    pub fn r33_x_value<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XValue>>> {
        span!("r33_x_value");
        store
            .iter_x_value()
            .filter(|x_value| x_value.lock().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
