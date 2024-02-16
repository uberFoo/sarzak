// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"format_bits-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression_bit::ExpressionBit;
use crate::v2::lu_dog_vec::types::format_string::FormatString;
use crate::v2::lu_dog_vec::types::string_bit::StringBit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatBits {
    pub subtype: FormatBitsEnum,
    pub id: usize,
    /// R111: [`FormatBits`] 'comprise' [`FormatString`]
    pub format_string: usize,
    /// R113: [`FormatBits`] 'next' [`FormatBits`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FormatBitsEnum {
    ExpressionBit(usize),
    StringBit(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-implementation"}}}
impl FormatBits {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-new_expression_bit"}}}
    /// Inter a new FormatBits in the store, and return it's `id`.
    pub fn new_expression_bit(
        format_string: &Rc<RefCell<FormatString>>,
        next: Option<&Rc<RefCell<FormatBits>>>,
        subtype: &Rc<RefCell<ExpressionBit>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<FormatBits>> {
        store.inter_format_bits(|id| {
            Rc::new(RefCell::new(FormatBits {
                format_string: format_string.borrow().id,
                next: next.map(|format_bits| format_bits.borrow().id),
                subtype: FormatBitsEnum::ExpressionBit(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-new_string_bit"}}}
    /// Inter a new FormatBits in the store, and return it's `id`.
    pub fn new_string_bit(
        format_string: &Rc<RefCell<FormatString>>,
        next: Option<&Rc<RefCell<FormatBits>>>,
        subtype: &Rc<RefCell<StringBit>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<FormatBits>> {
        store.inter_format_bits(|id| {
            Rc::new(RefCell::new(FormatBits {
                format_string: format_string.borrow().id,
                next: next.map(|format_bits| format_bits.borrow().id),
                subtype: FormatBitsEnum::StringBit(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-forward-to-format_string"}}}
    /// Navigate to [`FormatString`] across R111(1-*)
    pub fn r111_format_string<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FormatString>>> {
        vec![store.exhume_format_string(&self.format_string).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FormatBits`] across R113(1-*c)
    pub fn r113_format_bits<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FormatBits>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_format_bits(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-bi-cond-to-format_bits"}}}
    /// Navigate to [`FormatBits`] across R113(1c-1c)
    pub fn r113c_format_bits<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FormatBits>>> {
        let format_bits = store
            .iter_format_bits()
            .find(|format_bits| format_bits.borrow().next == Some(self.id));
        match format_bits {
            Some(ref format_bits) => vec![format_bits.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
    /// Navigate to [`FormatString`] across R112(1-1)
    pub fn r112_format_string<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<FormatString>>> {
        vec![store
            .iter_format_string()
            .find(|format_string| format_string.borrow().first_format_bit == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-struct-impl-nav-backward-one-to-format_string"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"format_bits-implementation"}}}
impl PartialEq for FormatBits {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.format_string == other.format_string
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
