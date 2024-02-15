// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"string_bit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::format_bits::FormatBits;
use crate::v2::lu_dog::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StringBit {
    pub id: Uuid,
    /// R108: [`StringBit`] 'refers to a' [`StringLiteral`]
    pub z_string: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-implementation"}}}
impl StringBit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-struct-impl-new"}}}
    /// Inter a new 'String Bit' in the store, and return it's `id`.
    pub fn new(
        z_string: &Rc<RefCell<StringLiteral>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<StringBit>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(StringBit {
            id,
            z_string: z_string.borrow().id,
        }));
        store.inter_string_bit(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-struct-impl-nav-forward-to-z_string"}}}
    /// Navigate to [`StringLiteral`] across R108(1-*)
    pub fn r108_string_literal<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<StringLiteral>>> {
        vec![store.exhume_string_literal(&self.z_string).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_bit-impl-nav-subtype-to-supertype-format_bits"}}}
    // Navigate to [`FormatBits`] across R110(isa)
    pub fn r110_format_bits<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<FormatBits>>> {
        vec![store.exhume_format_bits(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
