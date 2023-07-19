// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_ndrwlock_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_ndrwlock_vec::types::struct_expression::StructExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-documentation"}}}
/// A Struct Field Expression
///
/// This assigns a value to a field in a structure.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldExpression {
    pub id: usize,
    pub name: String,
    /// R38: [`FieldExpression`] '' [`Expression`]
    pub expression: usize,
    /// R26: [`FieldExpression`] 'belongs to a' [`StructExpression`]
    pub woog_struct: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-implementation"}}}
impl FieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-new"}}}
    /// Inter a new 'Field Expression' in the store, and return it's `id`.
    pub fn new(
        name: String,
        expression: &Arc<RwLock<Expression>>,
        woog_struct: &Arc<RwLock<StructExpression>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<FieldExpression>> {
        store.inter_field_expression(|id| {
            Arc::new(RwLock::new(FieldExpression {
                id,
                name: name.to_owned(),
                expression: expression.read().unwrap().id,
                woog_struct: woog_struct.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R38(1-*)
    pub fn r38_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r38_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`StructExpression`] across R26(1-*)
    pub fn r26_struct_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<StructExpression>>> {
        span!("r26_struct_expression");
        vec![store.exhume_struct_expression(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::FieldExpression(id) = expression.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-implementation"}}}
impl PartialEq for FieldExpression {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.expression == other.expression
            && self.woog_struct == other.woog_struct
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
