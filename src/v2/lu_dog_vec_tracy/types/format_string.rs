// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_string-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::format_bits::FormatBits;
use crate::v2::lu_dog_vec_tracy::types::literal::Literal;
use crate::v2::lu_dog_vec_tracy::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatString {
    pub id: usize,
    /// R112: [`FormatString`] 'needs to first' [`FormatBits`]
    pub first_format_bit: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-implementation"}}}
impl FormatString {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-new"}}}
    /// Inter a new 'Format String' in the store, and return it's `id`.
    pub fn new(
        first_format_bit: Option<&Rc<RefCell<FormatBits>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FormatString>> {
        store.inter_format_string(|id| {
            Rc::new(RefCell::new(FormatString {
                id,
                first_format_bit: first_format_bit.map(|format_bits| format_bits.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-list"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-first_format_bit"}}}
    /// Navigate to [`FormatBits`] across R112(1-*c)
    pub fn r112_format_bits<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FormatBits>>> {
        span!("r112_format_bits");
        match self.first_format_bit {
            Some(ref first_format_bit) => {
                vec![store.exhume_format_bits(&first_format_bit).unwrap()]
            }
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-forward-cond-to-format_bits"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-struct-impl-nav-backward-1_M-to-format_bits"}}}
    /// Navigate to [`FormatBits`] across R111(1-M)
    pub fn r111_format_bits<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FormatBits>>> {
        span!("r111_format_bits");
        store
            .iter_format_bits()
            .filter(|format_bits| format_bits.borrow().format_string == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Literal>>> {
        span!("r22_literal");
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::FormatString(id) = literal.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_string-implementation"}}}
impl PartialEq for FormatString {
    fn eq(&self, other: &Self) -> bool {
        self.first_format_bit == other.first_format_bit
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
