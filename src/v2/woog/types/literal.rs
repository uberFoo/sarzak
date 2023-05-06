// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-const-documentation"}}}
/// A Literal Expression
///
/// It's literally, a literal. Like that? 🤣
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-const-definition"}}}
pub const LITERAL: Uuid = uuid!["791d5acd-5374-5fde-b181-dd1e0c005a02"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Literal;

impl Literal {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        LITERAL
    }
}

impl Default for Literal {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
