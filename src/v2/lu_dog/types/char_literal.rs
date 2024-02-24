// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"char_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CharLiteral {
    pub id: Uuid,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-implementation"}}}
impl CharLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-struct-impl-new"}}}
    /// Inter a new 'Char Literal' in the store, and return it's `id`.
    pub fn new(x_value: i64, store: &mut LuDogStore) -> Rc<RefCell<CharLiteral>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(CharLiteral { id, x_value }));
        store.inter_char_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"char_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Literal>>> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::CharLiteral(id) = literal.borrow().subtype {
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
