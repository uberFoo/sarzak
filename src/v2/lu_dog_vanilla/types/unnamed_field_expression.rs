// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unnamed_field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::field_expression::FieldExpression;
use crate::v2::lu_dog_vanilla::types::field_expression::FieldExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UnnamedFieldExpression {
    pub id: Uuid,
    pub position: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-implementation"}}}
impl UnnamedFieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-struct-impl-new"}}}
    /// Inter a new 'Unnamed Field Expression' in the store, and return it's `id`.
    pub fn new(position: i64, store: &mut LuDogVanillaStore) -> UnnamedFieldExpression {
        let id = Uuid::new_v4();
        let new = UnnamedFieldExpression { id, position };
        store.inter_unnamed_field_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-impl-nav-subtype-to-supertype-field_expression"}}}
    // Navigate to [`FieldExpression`] across R94(isa)
    pub fn r94_field_expression<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&FieldExpression> {
        vec![store
            .iter_field_expression()
            .find(|field_expression| {
                if let FieldExpressionEnum::UnnamedFieldExpression(id) = field_expression.subtype {
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
