// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"integer_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-const-documentation"}}}
/// An Integer
///
/// I'm not sure what to do about width. I think I coded it as an i64 in the parser.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-const-definition"}}}
pub const INTEGER_LITERAL: Uuid = uuid!["a4ea76ae-a835-5858-a09b-232aa866abe7"];

pub struct IntegerLiteral;

impl IntegerLiteral {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        INTEGER_LITERAL
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
