// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-const-documentation"}}}
/// A statement that consists of just an expression.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-const-definition"}}}
pub const EXPRESSION_STATEMENT: Uuid = uuid!["28d7b340-b89a-5f1e-ab84-fd7dff79d9a1"];

pub struct ExpressionStatement;

impl ExpressionStatement {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        EXPRESSION_STATEMENT
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
