// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-const-documentation"}}}
/// Expression Statement
///
/// A [statement](https://doc.rust-lang.org/reference/statements.html#expression-statements
/// ) used for it's side effects.
///
/// I'd prefer to call this just `Expression`, but that would break things. Here's an [issue
///  to address the limitation.](https://git.uberfoo.com/sarzak/grace/-/issues/44)
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression_statement-const-definition"}}}
pub const EXPRESSION_STATEMENT: Uuid = uuid!["7cb01522-5a5c-5eff-9904-590a57d7b8c5"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExpressionStatement;

impl ExpressionStatement {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        EXPRESSION_STATEMENT
    }
}

impl Default for ExpressionStatement {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
