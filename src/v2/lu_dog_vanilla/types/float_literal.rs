// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"float_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::literal::Literal;
use crate::v2::lu_dog_vanilla::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-struct-documentation"}}}
/// A Floating Point Literal
///
/// Nothing fancy. No scientific notation.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FloatLiteral {
    pub id: Uuid,
    pub x_value: f64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-implementation"}}}
impl FloatLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-struct-impl-new"}}}
    /// Inter a new 'Float Literal' in the store, and return it's `id`.
    pub fn new(x_value: f64, store: &mut LuDogVanillaStore) -> FloatLiteral {
        let id = Uuid::new_v4();
        let new = FloatLiteral { id, x_value };
        store.inter_float_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Literal> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::FloatLiteral(id) = literal.subtype {
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
