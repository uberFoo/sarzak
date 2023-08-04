// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::block::Block;
use crate::v2::woog::types::call::Call;
use crate::v2::woog::types::literal::LITERAL;
use crate::v2::woog::types::x_let::XLet;
use crate::v2::woog::types::x_value::XValue;
use crate::v2::woog::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-documentation"}}}
/// An expression
///
/// An expression produces a value. There are all sorts of expressions, and I'm only going to
///  cover a very few for now.
///
/// The expressions here roughly align with what's found in [rust](https://doc.rust-lang.org
/// /reference/expressions.html).
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Expression {
    Block(Uuid),
    Call(Uuid),
    Literal(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-new-impl"}}}
    /// Create a new instance of Expression::Block
    pub fn new_block(block: &Rc<RefCell<Block>>, store: &mut WoogStore) -> Rc<RefCell<Self>> {
        let id = block.borrow().id;
        if let Some(block) = store.exhume_expression(&id) {
            block
        } else {
            let new = Rc::new(RefCell::new(Self::Block(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Call
    pub fn new_call(call: &Rc<RefCell<Call>>, store: &mut WoogStore) -> Rc<RefCell<Self>> {
        let id = call.borrow().id;
        if let Some(call) = store.exhume_expression(&id) {
            call
        } else {
            let new = Rc::new(RefCell::new(Self::Call(id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Literal
    pub fn new_literal(store: &WoogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_expression(&LITERAL).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Block(id) => *id,
            Self::Call(id) => *id,
            Self::Literal(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_let"}}}
    /// Navigate to [`XLet`] across R18(1-M)
    pub fn r18_x_let<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<XLet>>> {
        span!("r18_x_let");
        store
            .iter_x_let()
            .filter(|x_let| x_let.borrow().expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-value"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R7(isa)
    pub fn r7_x_value<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r7_x_value");
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Expression(id) = x_value.borrow().subtype {
                    id == self.id()
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
