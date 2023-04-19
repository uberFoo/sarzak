// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::statement::Statement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-documentation"}}}
/// A statement that consists of just an expression.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExpressionStatement {
    pub id: Uuid,
    /// R31: [`ExpressionStatement`] '' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-implementation"}}}
impl ExpressionStatement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-impl-new"}}}
    /// Inter a new 'Expression Statement' in the store, and return it's `id`.
    pub fn new(expression: &Expression, store: &mut LuDogStore) -> ExpressionStatement {
        let id = Uuid::new_v4();
        let new = ExpressionStatement {
            id: id,
            expression: expression.id(),
        };
        store.inter_expression_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-impl-new_"}}}
    /// Inter a new 'Expression Statement' in the store, and return it's `id`.
    pub fn new_(expression: &Expression) -> ExpressionStatement {
        let id = Uuid::new_v4();
        let new = ExpressionStatement {
            id: id,
            expression: expression.id(),
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R31(1-*)
    pub fn r31_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R16(isa)
    pub fn r16_statement<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Statement> {
        vec![store.exhume_statement(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
