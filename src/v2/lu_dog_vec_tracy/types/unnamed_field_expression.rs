// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unnamed_field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::field_expression::FieldExpression;
use crate::v2::lu_dog_vec_tracy::types::field_expression::FieldExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnnamedFieldExpression {
    pub id: usize,
    pub position: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-implementation"}}}
impl UnnamedFieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-struct-impl-new"}}}
    /// Inter a new 'Unnamed Field Expression' in the store, and return it's `id`.
    pub fn new(
        position: i64,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<UnnamedFieldExpression>> {
        store.inter_unnamed_field_expression(|id| {
            Rc::new(RefCell::new(UnnamedFieldExpression { id, position }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-impl-nav-subtype-to-supertype-field_expression"}}}
    // Navigate to [`FieldExpression`] across R94(isa)
    pub fn r94_field_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FieldExpression>>> {
        span!("r94_field_expression");
        vec![store
            .iter_field_expression()
            .find(|field_expression| {
                if let FieldExpressionEnum::UnnamedFieldExpression(id) =
                    field_expression.borrow().subtype
                {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unnamed_field_expression-implementation"}}}
impl PartialEq for UnnamedFieldExpression {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
