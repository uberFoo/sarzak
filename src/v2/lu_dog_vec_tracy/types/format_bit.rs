// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_bit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_vec_tracy::types::format_string::FormatString;
use crate::v2::lu_dog_vec_tracy::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatBit {
    pub subtype: FormatBitEnum,
    pub id: usize,
    /// R111: [`FormatBit`] 'comprise' [`FormatString`]
    pub format_string: usize,
    /// R113: [`FormatBit`] 'next' [`FormatBit`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FormatBitEnum {
    ExpressionBit(usize),
    StringBit(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-implementation"}}}
impl FormatBit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-new_expression_bit"}}}
    /// Inter a new FormatBit in the store, and return it's `id`.
    pub fn new_expression_bit(
        format_string: &Rc<RefCell<FormatString>>,
        next: Option<&Rc<RefCell<FormatBit>>>,
        subtype: &Rc<RefCell<ExpressionBit>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FormatBit>> {
        store.inter_format_bit(|id| {
            Rc::new(RefCell::new(FormatBit {
                format_string: format_string.borrow().id,
                next: next.map(|format_bit| format_bit.borrow().id),
                subtype: FormatBitEnum::ExpressionBit(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-new_string_bit"}}}
    /// Inter a new FormatBit in the store, and return it's `id`.
    pub fn new_string_bit(
        format_string: &Rc<RefCell<FormatString>>,
        next: Option<&Rc<RefCell<FormatBit>>>,
        subtype: &Rc<RefCell<StringBit>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FormatBit>> {
        store.inter_format_bit(|id| {
            Rc::new(RefCell::new(FormatBit {
                format_string: format_string.borrow().id,
                next: next.map(|format_bit| format_bit.borrow().id),
                subtype: FormatBitEnum::StringBit(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-forward-to-format_string"}}}
    /// Navigate to [`FormatString`] across R111(1-*)
    pub fn r111_format_string<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FormatString>>> {
        span!("r111_format_string");
        vec![store.exhume_format_string(&self.format_string).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FormatBit`] across R113(1-*c)
    pub fn r113_format_bit<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FormatBit>>> {
        span!("r113_format_bit");
        match self.next {
            Some(ref next) => vec![store.exhume_format_bit(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-backward-one-bi-cond-to-format_bit"}}}
    /// Navigate to [`FormatBit`] across R113(1c-1c)
    pub fn r113c_format_bit<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FormatBit>>> {
        span!("r113_format_bit");
        let format_bit = store
            .iter_format_bit()
            .find(|format_bit| format_bit.borrow().next == Some(self.id));
        match format_bit {
            Some(ref format_bit) => vec![format_bit.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R112(1-1)
    pub fn r112_format_string<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FormatString>>> {
        span!("r112_format_string");
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.borrow().first_format_bit == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bit-implementation"}}}
impl PartialEq for FormatBit {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.format_string == other.format_string
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
