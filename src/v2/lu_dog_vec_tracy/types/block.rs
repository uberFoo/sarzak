// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::body::Body;
use crate::v2::lu_dog_vec_tracy::types::body::BodyEnum;
use crate::v2::lu_dog_vec_tracy::types::expression::Expression;
use crate::v2::lu_dog_vec_tracy::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec_tracy::types::statement::Statement;
use crate::v2::lu_dog_vec_tracy::types::x_if::XIf;
use crate::v2::lu_dog_vec_tracy::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-documentation"}}}
/// A Block Expression
///
/// A block expression is an expression that consists of an ordered set of statements, living
///  between an opening `{`, and a closing `}`.
///
/// Given that it's an expression it has a Type. The type is the value of the last expression
///  in the block, if it's not closed by a `;`. If the last statement is terminated thusly, then
///  the value is `[Empty]`, or `()`.
///
/// The `bug` attribute is just there to keep this thing unique. An issue that needs addressing
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub a_sink: bool,
    pub bug: Uuid,
    pub id: usize,
    /// R93: [`Block`] '' [`Block`]
    pub parent: Option<usize>,
    /// R71: [`Block`] '' [`Statement`]
    pub statement: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub fn new(
        a_sink: bool,
        bug: Uuid,
        parent: Option<&Rc<RefCell<Block>>>,
        statement: Option<&Rc<RefCell<Statement>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Block>> {
        store.inter_block(|id| {
            Rc::new(RefCell::new(Block {
                a_sink,
                bug,
                id,
                parent: parent.map(|block| block.borrow().id),
                statement: statement.map(|statement| statement.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-parent"}}}
    /// Navigate to [`Block`] across R93(1-*c)
    pub fn r93_block<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Block>>> {
        span!("r93_block");
        match self.parent {
            Some(ref parent) => vec![store.exhume_block(&parent).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R71(1-*c)
    pub fn r71_statement<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Statement>>> {
        span!("r71_statement");
        match self.statement {
            Some(ref statement) => vec![store.exhume_statement(&statement).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R93(1c-1c)
    pub fn r93c_block<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Block>>> {
        span!("r93_block");
        let block = store
            .iter_block()
            .find(|block| block.borrow().parent == Some(self.id));
        match block {
            Some(ref block) => vec![block.clone()],
            None => Vec::new(),
        }
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-for_loop"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R46(1-M)
    pub fn r46_x_if<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XIf>>> {
        span!("r46_x_if");
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().true_block == self.id)
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_Mc-to-x_if"}}}
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub fn r18_statement<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Statement>>> {
        span!("r18_statement");
        store
            .iter_statement()
            .filter(|statement| statement.borrow().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R33(1-M)
    pub fn r33_x_value<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r33_x_value");
        store
            .iter_x_value()
            .filter(|x_value| x_value.borrow().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub fn r80_body<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Body>>> {
        span!("r80_body");
        vec![store
            .iter_body()
            .find(|body| {
                if let BodyEnum::Block(id) = body.borrow().subtype {
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
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Block(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.a_sink == other.a_sink
            && self.bug == other.bug
            && self.parent == other.parent
            && self.statement == other.statement
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
