// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::statement::Statement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-documentation"}}}
/// A Block Expression
///
/// A block expression is an expression that consists of an ordered set of statements, living
/// between an opening `{`, and a closing `}`.
///
///  Given that it's an expression it has a Type. The type is the value of the last expression
/// in the block, if it's not closed by a `;`. If the last statement is termintat thusly, then
/// the value is `[Empty]`, or `()`.
///
/// The `bug` attribute is there to force the compiler to generate code. Apparently there's
/// some bug in grace that's causing this to be generated as a const. I don't want to get into
/// it, and this is the most expedient solution.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub bug: Uuid,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new(bug: Uuid, store: &mut LuDogStore) -> Block {
        let id = Uuid::new_v4();
        let new = Block { bug: bug, id: id };
        store.inter_block(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new_"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new_(bug: Uuid) -> Block {
        let id = Uuid::new_v4();
        let new = Block { bug: bug, id: id };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-function"}}}
    /// Navigate to [`Function`] across R19(1-1c)
    pub fn r19c_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Function> {
        let function = store
            .iter_function()
            .find(|function| function.block == self.id);
        match function {
            Some(ref function) => vec![function],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub fn r18_statement<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Statement> {
        store
            .iter_statement()
            .filter_map(|statement| {
                if statement.block == self.id {
                    Some(statement)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
