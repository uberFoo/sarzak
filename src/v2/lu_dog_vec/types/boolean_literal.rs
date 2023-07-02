// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::false_literal::FALSE_LITERAL;
use crate::v2::lu_dog_vec::types::literal::Literal;
use crate::v2::lu_dog_vec::types::literal::LiteralEnum;
use crate::v2::lu_dog_vec::types::true_literal::TRUE_LITERAL;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-hybrid-documentation"}}}
/// A Boolean
///
/// It's either `true` or `false`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BooleanLiteral {
    pub subtype: BooleanLiteralEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BooleanLiteralEnum {
    FalseLiteral(Uuid),
    TrueLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-implementation"}}}
impl BooleanLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-struct-impl-new_false_literal"}}}
    /// Inter a new BooleanLiteral in the store, and return it's `id`.
    pub fn new_false_literal(store: &mut LuDogVecStore) -> Rc<RefCell<BooleanLiteral>> {
        store.inter_boolean_literal(|id| {
            Rc::new(RefCell::new(BooleanLiteral {
                subtype: BooleanLiteralEnum::FalseLiteral(FALSE_LITERAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-struct-impl-new_true_literal"}}}
    /// Inter a new BooleanLiteral in the store, and return it's `id`.
    pub fn new_true_literal(store: &mut LuDogVecStore) -> Rc<RefCell<BooleanLiteral>> {
        store.inter_boolean_literal(|id| {
            Rc::new(RefCell::new(BooleanLiteral {
                subtype: BooleanLiteralEnum::TrueLiteral(TRUE_LITERAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Literal>>> {
        span!("r22_literal");
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::BooleanLiteral(id) = literal.borrow().subtype {
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
