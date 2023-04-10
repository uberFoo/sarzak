// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::block::Block;
use crate::v2::woog::types::call::Call;
use crate::v2::woog::types::literal::LITERAL;
use crate::v2::woog::types::value::Value;
use crate::v2::woog::types::x_let::XLet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-documentation"}}}
/// An expression
///
/// An expression produces a value. There are all sorts of expressions, and I'm only going to
/// cover a very few for now.
///
/// The expressions here roughly align with what's found in [rust](https://doc.rust-lang.org
////reference/expressions.html).
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Expression {
    Block(Uuid),
    Call(Uuid),
    Literal(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-new-impl"}}}
    /// Create a new instance of Expression::Block
    pub fn new_block(block: &Block, store: &mut WoogStore) -> Self {
        let new = Self::Block(block.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_block_(block: &Block) -> Self {
        let new = Self::Block(block.id);
        new
    }

    /// Create a new instance of Expression::Call
    pub fn new_call(call: &Call, store: &mut WoogStore) -> Self {
        let new = Self::Call(call.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_call_(call: &Call) -> Self {
        let new = Self::Call(call.id);
        new
    }

    /// Create a new instance of Expression::Literal
    pub fn new_literal() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Literal(LITERAL)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Expression::Block(id) => *id,
            Expression::Call(id) => *id,
            Expression::Literal(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_let"}}}
    /// Navigate to [`XLet`] across R18(1-M)
    pub fn r18_x_let<'a>(&'a self, store: &'a WoogStore) -> Vec<&XLet> {
        store
            .iter_x_let()
            .filter_map(|x_let| {
                if x_let.expression == self.id() {
                    Some(x_let)
                // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
                // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-struct_expression_field"}}}
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-value"}}}
    // Navigate to [`Value`] across R7(isa)
    pub fn r7_value<'a>(&'a self, store: &'a WoogStore) -> Vec<&Value> {
        vec![store.exhume_value(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
