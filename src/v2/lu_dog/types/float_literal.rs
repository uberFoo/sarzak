// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"float_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-const-documentation"}}}
/// A Floating Point Literal
///
/// Nothing fancy. No scientific notation.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"float_literal-const-definition"}}}
pub const FLOAT_LITERAL: Uuid = uuid!["32cde28d-7552-5fca-bc79-0a9fb105ea77"];

pub struct FloatLiteral;

impl FloatLiteral {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        FLOAT_LITERAL
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
