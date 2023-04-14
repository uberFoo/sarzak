// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"result_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-const-documentation"}}}
/// An Expression Statement that is not terminated by a semi-colon, and this yields a result
///. This is only applicable if it's the last statement in a block.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-const-definition"}}}
pub const RESULT_STATEMENT: Uuid = uuid!["763f4f22-d8a7-5993-8bfd-4d46cc0377b1"];

pub struct ResultStatement;

impl ResultStatement {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        RESULT_STATEMENT
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
