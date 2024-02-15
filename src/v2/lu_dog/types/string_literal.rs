// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"string_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::literal::LiteralEnum;
use crate::v2::lu_dog::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-documentation"}}}
/// A String
///
/// A string is a set of characters enclosed in double quotes. Strings are unicode strings encoded
///  as UTF-8.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StringLiteral {
    pub id: Uuid,
    pub x_value: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-implementation"}}}
impl StringLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-impl-new"}}}
    /// Inter a new 'String Literal' in the store, and return it's `id`.
    pub fn new(x_value: String, store: &mut LuDogStore) -> Rc<RefCell<StringLiteral>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(StringLiteral { id, x_value }));
        store.inter_string_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-impl-nav-backward-1_M-to-string_bit"}}}
    /// Navigate to [`StringBit`] across R108(1-M)
    pub fn r108_string_bit<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<StringBit>>> {
        store
            .iter_string_bit()
            .filter(|string_bit| string_bit.borrow().z_string == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Literal>>> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::StringLiteral(id) = literal.borrow().subtype {
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
