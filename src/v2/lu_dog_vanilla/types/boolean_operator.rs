// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::and::AND;
use crate::v2::lu_dog_vanilla::types::binary::Binary;
use crate::v2::lu_dog_vanilla::types::binary::BinaryEnum;
use crate::v2::lu_dog_vanilla::types::or::OR;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-documentation"}}}
/// A Boolean Operaator
///
/// There are two — || and &&.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BooleanOperator {
    pub subtype: BooleanOperatorEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BooleanOperatorEnum {
    And(Uuid),
    Or(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-implementation"}}}
impl BooleanOperator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-struct-impl-new_and"}}}
    /// Inter a new BooleanOperator in the store, and return it's `id`.
    pub fn new_and(bogus: bool, store: &mut LuDogVanillaStore) -> BooleanOperator {
        let id = Uuid::new_v4();
        let new = BooleanOperator {
            bogus: bogus,
            subtype: BooleanOperatorEnum::And(AND),
            id,
        };
        store.inter_boolean_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-struct-impl-new_or"}}}
    /// Inter a new BooleanOperator in the store, and return it's `id`.
    pub fn new_or(bogus: bool, store: &mut LuDogVanillaStore) -> BooleanOperator {
        let id = Uuid::new_v4();
        let new = BooleanOperator {
            bogus: bogus,
            subtype: BooleanOperatorEnum::Or(OR),
            id,
        };
        store.inter_boolean_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_operator-impl-nav-subtype-to-supertype-binary"}}}
    // Navigate to [`Binary`] across R48(isa)
    pub fn r48_binary<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Binary> {
        vec![store
            .iter_binary()
            .find(|binary| {
                if let BinaryEnum::BooleanOperator(id) = binary.subtype {
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
