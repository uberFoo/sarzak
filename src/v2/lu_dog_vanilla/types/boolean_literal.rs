// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"boolean_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::false_literal::FALSE_LITERAL;
use crate::v2::lu_dog_vanilla::types::literal::Literal;
use crate::v2::lu_dog_vanilla::types::literal::LiteralEnum;
use crate::v2::lu_dog_vanilla::types::true_literal::TRUE_LITERAL;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
    pub bogus: bool,
    pub id: Uuid,
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
    pub fn new_false_literal(bogus: bool, store: &mut LuDogVanillaStore) -> BooleanLiteral {
        let id = Uuid::new_v4();
        let new = BooleanLiteral {
            bogus: bogus,
            subtype: BooleanLiteralEnum::FalseLiteral(FALSE_LITERAL),
            id,
        };
        store.inter_boolean_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-struct-impl-new_true_literal"}}}
    /// Inter a new BooleanLiteral in the store, and return it's `id`.
    pub fn new_true_literal(bogus: bool, store: &mut LuDogVanillaStore) -> BooleanLiteral {
        let id = Uuid::new_v4();
        let new = BooleanLiteral {
            bogus: bogus,
            subtype: BooleanLiteralEnum::TrueLiteral(TRUE_LITERAL),
            id,
        };
        store.inter_boolean_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"boolean_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Literal> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::BooleanLiteral(id) = literal.subtype {
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
