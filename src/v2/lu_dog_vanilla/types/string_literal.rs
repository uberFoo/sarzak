// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"string_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::literal::Literal;
use crate::v2::lu_dog_vanilla::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
    pub fn new(x_value: String, store: &mut LuDogVanillaStore) -> StringLiteral {
        let id = Uuid::new_v4();
        let new = StringLiteral { id, x_value };
        store.inter_string_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"string_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Literal> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::StringLiteral(id) = literal.subtype {
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
