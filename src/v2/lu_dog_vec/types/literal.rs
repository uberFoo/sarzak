// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_vec::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_vec::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
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
        subtype: &Rc<RefCell<BooleanLiteral>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                subtype: LiteralEnum::BooleanLiteral(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_float_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_float_literal(
        subtype: &Rc<RefCell<FloatLiteral>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                subtype: LiteralEnum::FloatLiteral(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_integer_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_integer_literal(
        subtype: &Rc<RefCell<IntegerLiteral>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                subtype: LiteralEnum::IntegerLiteral(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_string_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_string_literal(
        subtype: &Rc<RefCell<StringLiteral>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                subtype: LiteralEnum::StringLiteral(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Literal(id) = expression.borrow().subtype {
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