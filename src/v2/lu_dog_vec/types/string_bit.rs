// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"string_bit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::format_bit::FormatBit;
use crate::v2::lu_dog_vec::types::format_bit::FormatBitEnum;
use crate::v2::lu_dog_vec::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringBit {
    pub id: usize,
    /// R108: [`StringBit`] 'refers to a' [`StringLiteral`]
    pub z_string: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-implementation"}}}
impl StringBit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-struct-impl-new"}}}
    /// Inter a new 'String Bit' in the store, and return it's `id`.
    pub fn new(
        z_string: &Rc<RefCell<StringLiteral>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<StringBit>> {
        store.inter_string_bit(|id| {
            Rc::new(RefCell::new(StringBit {
                id,
                z_string: z_string.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-struct-impl-nav-forward-to-z_string"}}}
    /// Navigate to [`StringLiteral`] across R108(1-*)
    pub fn r108_string_literal<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<StringLiteral>>> {
        vec![store.exhume_string_literal(&self.z_string).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-impl-nav-subtype-to-supertype-format_bits"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-impl-nav-subtype-to-supertype-format_bit"}}}
    // Navigate to [`FormatBit`] across R110(isa)
    pub fn r110_format_bit<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<FormatBit>>> {
        vec![store
            .iter_format_bit()
            .find(|format_bit| {
                if let FormatBitEnum::StringBit(id) = format_bit.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-implementation"}}}
impl PartialEq for StringBit {
    fn eq(&self, other: &Self) -> bool {
        self.z_string == other.z_string
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
