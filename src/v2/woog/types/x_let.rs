// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_let-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog::UUID_NS;

// Referrer imports
use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::variable::Variable;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-struct-documentation"}}}
/// Let Statement
///
/// A means of assigning a variable to an expression. I don't think that I'll ever deal with
/// the pattern stuff [a full implementation](https://doc.rust-lang.org/reference/statements.html
///#let-statements) would require.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XLet {
    pub id: Uuid,
    pub value: String,
    /// R18: [`XLet`] 'to a variable assigns' [`Expression`]
    pub expression: Uuid,
    /// R17: [`XLet`] 'gives value to a' [`Variable`]
    pub variable: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-implementation"}}}
impl XLet {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-struct-impl-new"}}}
    /// Inter a new XLet in the store, and return it's `id`.
    pub fn new(
        value: String,
        expression: &Expression,
        variable: &Variable,
        store: &mut WoogStore,
    ) -> XLet {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}", value, expression, variable).as_bytes(),
        );
        let new = XLet {
            value: value,
            expression: expression.id(),
            variable: variable.id(),
            id,
        };
        store.inter_x_let(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R18(1-*)
    pub fn r18_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_let-struct-impl-nav-forward-to-variable"}}}
    /// Navigate to [`Variable`] across R17(1-*)
    pub fn r17_variable<'a>(&'a self, store: &'a WoogStore) -> Vec<&Variable> {
        vec![store.exhume_variable(&self.variable).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
