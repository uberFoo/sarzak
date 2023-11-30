// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::body::Body;
use crate::v2::lu_dog_vec::types::body::BodyEnum;
use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec::types::for_loop::ForLoop;
use crate::v2::lu_dog_vec::types::statement::Statement;
use crate::v2::lu_dog_vec::types::x_if::XIf;
use crate::v2::lu_dog_vec::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
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
        store: &mut LuDogVecStore,
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-next"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-parent"}}}
    /// Navigate to [`Block`] across R93(1-*c)
    pub fn r93_block<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Block>>> {
        match self.parent {
            Some(ref parent) => vec![store.exhume_block(&parent).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R71(1-*c)
    pub fn r71_statement<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Statement>>> {
        match self.statement {
            Some(ref statement) => vec![store.exhume_statement(&statement).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R93(1c-1c)
    pub fn r93c_block<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Block>>> {
        let block = store
            .iter_block()
            .find(|block| block.borrow().parent == Some(self.id));
        match block {
            Some(ref block) => vec![block.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub fn r43_for_loop<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ForLoop>>> {
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.borrow().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-function"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-function"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R46(1-M)
    pub fn r46_x_if<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XIf>>> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().true_block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub fn r52_x_if<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XIf>>> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.borrow().false_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-lambda"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-lambda"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub fn r18_statement<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Statement>>> {
        store
            .iter_statement()
            .filter(|statement| statement.borrow().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R33(1-M)
    pub fn r33_x_value<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XValue>>> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.borrow().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub fn r80_body<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Body>>> {
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
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
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
