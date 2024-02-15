// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"string_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::literal::Literal;
use crate::v2::lu_dog_vec_tracy::types::literal::LiteralEnum;
use crate::v2::lu_dog_vec_tracy::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-documentation"}}}
/// A String
///
/// A string is a set of characters enclosed in double quotes. Strings are unicode strings encoded
///  as UTF-8.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringLiteral {
    pub id: usize,
    pub x_value: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-implementation"}}}
impl StringLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-impl-new"}}}
    /// Inter a new 'String Literal' in the store, and return it's `id`.
    pub fn new(x_value: String, store: &mut LuDogVecTracyStore) -> Rc<RefCell<StringLiteral>> {
        store.inter_string_literal(|id| {
            Rc::new(RefCell::new(StringLiteral {
                id,
                x_value: x_value.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-struct-impl-nav-backward-1_M-to-string_bit"}}}
    /// Navigate to [`StringBit`] across R108(1-M)
    pub fn r108_string_bit<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<StringBit>>> {
        span!("r108_string_bit");
        store
            .iter_string_bit()
            .filter(|string_bit| string_bit.borrow().z_string == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Literal>>> {
        span!("r22_literal");
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-implementation"}}}
impl PartialEq for StringLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
